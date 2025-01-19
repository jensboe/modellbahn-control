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
use track::PowerState;


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

    let track_layout = track_layout::get_track_layout();

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

