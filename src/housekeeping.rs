pub mod housekeeping {
    use defmt::trace;
    use embassy_stm32::gpio::Output;
    use embassy_time::{Duration, Ticker};

    #[embassy_executor::task]
    pub async fn alive(mut led: Output<'static>) {
        let mut ticker = Ticker::every(Duration::from_secs(1));
        loop {
            led.toggle();
            trace!("Alive!");
            ticker.next().await;
        }
    }
}
