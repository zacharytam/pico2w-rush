#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_net::{Config, Stack, StackResources};
use embassy_rp::init;
use embassy_time::Timer;
use panic_probe as _;
use static_cell::StaticCell;

use cyw43::{Control, State};
use cyw43_pio::PioSpi;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    info!("Initializing Pico 2 W Wi-Fi...");

    let p = init(Default::default());

    // CYW43 driver setup
    let fw = include_bytes!("cyw43_fw.bin");
    let clm = include_bytes!("cyw43_clm.blob");
    static STATE: StaticCell<State> = StaticCell::new();
    let state = STATE.init(State::new());
    let pwr = p.PIN_23;
    let cs = p.PIN_25;
    let sck = p.PIN_29;
    let dio = p.PIN_28;

    let spi = PioSpi::new(
        p.PIO0, p.DMA_CH0, sck, dio, cs,
    );

    let mut control = Control::new(state, spi, pwr).await;
    control.init(fw, clm).await;

    // Create Access Point
    control.start_ap(b"Pico2W_AP", b"12345678", 6).await.unwrap();
    info!("Access Point started. SSID: Pico2W_AP, Password: 12345678");

    // Keep running
    loop {
        Timer::after_secs(5).await;
        info!("Wi-Fi still active...");
    }
}
