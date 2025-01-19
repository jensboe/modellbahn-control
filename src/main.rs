#![no_std]
#![no_main]

use defmt::*;
use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_stm32::gpio::{Level, Output, Pin, Speed};
use embassy_stm32::spi;
use embassy_stm32::time::Hertz;
use embassy_time::{Duration, Timer};
use panic_probe as _;

mod blockdriver;
mod housekeeping;
mod track;

mod track_layout;
use track::buffer::Buffer;
use track::straight::Straight;
use track::switch::Switch;
use track::{PowerState, Track};


#[embassy_executor::main]
async fn main(spawner: Spawner) {
    println!("Hello, modellbahn speaking!");
    info!("{}", env!("VERSION_STRING"));
    info!("Start tasks!");
    let p = embassy_stm32::init(Default::default());

    let led = Output::new(p.PB7.degrade(), Level::High, Speed::Low);

    spawner
        .spawn(housekeeping::housekeeping::alive(led))
        .unwrap();

    let mut spi_config = spi::Config::default();
    spi_config.frequency = Hertz(1_000_000);
    let spi = spi::Spi::new_blocking(p.SPI3, p.PC10, p.PC12, p.PC11, spi_config);

    let cs = Output::new(p.PD2, Level::Low, Speed::VeryHigh);

    spawner
        .spawn(blockdriver::blockdriver::refresh(spi, cs))
        .unwrap();


    info!("All tasks spawned!");
    let track_layout: &mut [&mut dyn Track]  = &mut [
        &mut Switch::new("A_d", "D_1a", "A_c", "A_3a", 100, 100),
        &mut Switch::new("A_c", "A_d", "A_1a", "A_2a", 100, 100),
        &mut Straight::new("A_1a", "A_c", "A_1b", 1_500),
        &mut Straight::new_platform("A_1b", "A_1a", "A_a", 200),
        &mut Switch::new("A_a", "B_1a", "A_1b", "A_b", 100, 100),

        &mut Straight::new("A_2a", "A_c", "A_2b", 1_500),
        &mut Straight::new_platform("A_2b", "A_2a", "A_b", 200),
        &mut Switch::new("A_b", "A_a", "A_3b", "A_2b", 100, 100),

        &mut Straight::new("A_3a", "A_d", "A_3b", 1_500),
        &mut Straight::new_platform("A_3b", "A_3a", "A_b", 200),

        &mut Straight::new("B_1a", "A_a", "C_a", 6_000),

        &mut Switch::new("C_a", "B_1a", "C_2a", "C_1a", 100, 100),
        &mut Straight::new("C_1a", "C_a", "C_1b", 1_500),
        &mut Straight::new_platform("C_1b", "C_1a", "C_b", 200),
        &mut Straight::new("C_2a", "C_a", "C_2b", 1_500),
        &mut Straight::new_platform("C_2b", "C_2a", "C_b", 200),

        &mut Switch::new("C_b", "C_c", "C_1b", "C_2b", 100, 100),
        &mut Switch::new("C_c", "D_1a", "C_b", "C_3c", 100, 100),
        &mut Buffer::new("C_3a", "C_3b", 200),
        &mut Straight::new_platform("C_3b", "C_3a", "C_3c", 200),
        &mut Straight::new("C_3c", "C_3b", "C_c", 200),

        &mut Straight::new("D_1a", "C_c", "A_d", 6_000),
    ];
    for track in track_layout.iter_mut() {
        track.set_power_state(PowerState::Off);
    }

    let mut current_track_id = "C_3b";
    let mut previous_track_id = "C_3a";
    let mut next_track_id = "";
    let mut wait_time = 0;
    loop {
        for track in track_layout.iter_mut() {
            if track.id() == current_track_id {
                track.set_power_state(PowerState::On);
                next_track_id = track.next_track(previous_track_id);
                wait_time = track.length();
                debug!("Driving from {} to {}, will take {}ms", track.id(), next_track_id, wait_time);
            }
        }
        for track in track_layout.iter_mut() {
            if track.id() == previous_track_id {
                track.set_power_state(PowerState::Off);
            }
        }

        Timer::after(Duration::from_millis(wait_time.into())).await;
        previous_track_id = current_track_id;
        current_track_id = next_track_id;
    }    
}

