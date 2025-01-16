#![no_std]
#![no_main]

use defmt::*;
use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_stm32::gpio::{Level, Output, Pin, Speed};
use embassy_stm32::spi;
use embassy_stm32::time::Hertz;
use panic_probe as _;

mod blockdriver;
mod housekeeping;

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

    let spi_config = spi::Config::default();
    let spi = spi::Spi::new_blocking(p.SPI3, p.PC10, p.PC12, p.PC11, spi_config);

    let cs = Output::new(p.PD2, Level::Low, Speed::VeryHigh);

    spawner
        .spawn(blockdriver::blockdriver::refresh(spi, cs))
        .unwrap();
    info!("All tasks spawned!");
}
