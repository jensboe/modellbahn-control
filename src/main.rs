#![no_std]
#![no_main]

use defmt::*;
use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_stm32::gpio::{Level, Output, Pin, Speed};
use embassy_stm32::spi;
use embassy_stm32::time::Hertz;
use embassy_time::{Duration, Ticker};
use panic_probe as _;

mod housekeeping;

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());

    spawner
        .spawn(housekeeping::housekeeping::alive(p.PB7.degrade()))
        .unwrap();

    let mut spi_config = spi::Config::default();
    spi_config.frequency = Hertz(1_000_000);

    let mut spi = spi::Spi::new_blocking(p.SPI3, p.PC10, p.PC12, p.PC11, spi_config);

    let mut cs = Output::new(p.PD2, Level::Low, Speed::VeryHigh);
    let mut ticker = Ticker::every(Duration::from_millis(50));

    loop {
        let mut buf = [0x0Au8; 4];
        cs.set_low();
        unwrap!(spi.blocking_transfer_in_place(&mut buf));
        cs.set_high();
        info!("Refreshing I/O: {=[u8]:x}", buf);
        ticker.next().await;
    }
}
