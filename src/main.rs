#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::gpio::{Level, Output, Speed};
use embassy_stm32::spi;
use embassy_stm32::time::Hertz;
use embassy_time::Timer;
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());
    let mut led = Output::new(p.PB7, Level::High, Speed::Low);

    let mut spi_config = spi::Config::default();
    spi_config.frequency = Hertz(1_000_000);
    
    let mut spi = spi::Spi::new_blocking(
        p.SPI3,
        p.PC10,
        p.PC12,
        p.PC11,
        spi_config,
    );

    let mut cs = Output::new(p.PD2, Level::Low, Speed::VeryHigh);

    loop {
        led.set_high();
        Timer::after_millis(300).await;
        led.set_low();
        Timer::after_millis(300).await;
        let mut buf = [0x0Au8; 4];
        cs.set_low();
        unwrap!(spi.blocking_transfer_in_place(&mut buf));
        cs.set_high();
        info!("Read: {=[u8]:x}", buf);
    }
}