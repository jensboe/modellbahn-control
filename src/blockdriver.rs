pub mod blockdriver {
    use defmt::*;
    use defmt_rtt as _;
    use embassy_stm32::gpio::Output;
    use embassy_stm32::mode::Blocking;
    use embassy_stm32::spi;
    use embassy_time::{Duration, Ticker};
    use panic_probe as _;
    #[embassy_executor::task]
    pub async fn refresh(mut spi: spi::Spi<'static, Blocking>, mut cs: Output<'static>) {
        let mut ticker = Ticker::every(Duration::from_millis(20));

        loop {
            let mut buf = [0x0Au8; 4];
            cs.set_low();
            unwrap!(spi.blocking_transfer_in_place(&mut buf));
            cs.set_high();
            trace!("Refreshing I/O: {=[u8]:x}", buf);
            ticker.next().await;
        }
    }
}
