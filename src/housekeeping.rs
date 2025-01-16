pub mod housekeeping {
    use defmt::{error, trace, warn, info};
    use embassy_stm32::gpio::{AnyPin, Level, Output, Speed};
    use embassy_time::{Duration, Ticker};


    #[embassy_executor::task]
    pub async fn alive(pin: AnyPin) {
        let mut led = Output::new(pin, Level::High, Speed::Low);
        let mut ticker = Ticker::every(Duration::from_secs(1));
        loop {
            led.toggle();
            trace!("Good to know: System is alive!");
            error!("Test");
            info!("Test");
            warn!("Test");
            ticker.next().await;
        }
    }
}
