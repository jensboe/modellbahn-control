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
use track_layout::TRACK_LAYOUT;

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


    let mut previous_track = TRACK_LAYOUT.iter().find(|track| track.id() == "C_3a").unwrap();
    let mut current_track = TRACK_LAYOUT.iter().find(|track| track.id() == "C_3b").unwrap();
    let mut next_track = current_track.get_next_track(previous_track.id());
    loop {
        previous_track = current_track;
        current_track = TRACK_LAYOUT.iter().find(|track| track.id() == next_track).unwrap();
        current_track.debug_print();
        Timer::after(Duration::from_millis(current_track.length().into())).await;
        next_track = current_track.get_next_track(previous_track.id());
        debug!("Next:    {}", next_track);
    }
}
