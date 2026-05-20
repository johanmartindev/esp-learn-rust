#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]
#![deny(clippy::large_stack_frames)]

use esp_hal::timer::timg::TimerGroup;
use esp_hal::{
    clock::CpuClock,
    gpio::{Level, Output},
    main,
    time::{Duration, Instant},
};
use esp_radio::ble::controller::BleConnector;
use esp_radio::wifi::{Config as WifiConfig, sta::StationConfig};

use log::error;
use log::info;

#[panic_handler]
fn panic(panic_info: &core::panic::PanicInfo) -> ! {
    error!("{}", panic_info);
    loop {}
}

extern crate alloc;

// This creates a default app-descriptor required by the esp-idf bootloader.
// For more information see: <https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/system/app_image_format.html#application-description>
esp_bootloader_esp_idf::esp_app_desc!();

#[allow(
    clippy::large_stack_frames,
    reason = "it's not unusual to allocate larger buffers etc. in main"
)]
#[main]
fn main() -> ! {
    // generator version: 1.3.0
    // generator parameters: --chip esp32 -o wokwi -o vscode -o esp32-wroom-da -o unstable-hal -o alloc -o wifi -o ble-bleps -o log

    esp_println::logger::init_logger_from_env();

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);
    let wifi_ssid = option_env!("WIFI_SSID").expect("Set WIFI_SSID in your local environment");
    let wifi_password =
        option_env!("WIFI_PASSWORD").expect("Set WIFI_PASSWORD in your local environment");

    // The following pins are used to bootstrap the chip. They are available
    // for use, but check the datasheet of the module for more information on them.
    // - GPIO0
    // - GPIO2
    // - GPIO5
    // - GPIO12
    // - GPIO15
    // These GPIO pins are in use by some feature of the module and should not be used.
    let _ = peripherals.GPIO6;
    let _ = peripherals.GPIO7;
    let _ = peripherals.GPIO8;
    let _ = peripherals.GPIO9;
    let _ = peripherals.GPIO10;
    let _ = peripherals.GPIO11;
    let _ = peripherals.GPIO16;
    let _ = peripherals.GPIO20;

    esp_alloc::heap_allocator!(#[esp_hal::ram(reclaimed)] size: 98768);
    // COEX needs more RAM - so we've added some more
    esp_alloc::heap_allocator!(size: 64 * 1024);

    let timg0 = TimerGroup::new(peripherals.TIMG0);
    let sw_interrupt =
        esp_hal::interrupt::software::SoftwareInterruptControl::new(peripherals.SW_INTERRUPT);
    esp_rtos::start(timg0.timer0, sw_interrupt.software_interrupt0);
    let (mut wifi_controller, _interfaces) =
        esp_radio::wifi::new(peripherals.WIFI, Default::default())
            .expect("Failed to initialize Wi-Fi controller");

    let wifi_config = WifiConfig::Station(
        StationConfig::default()
            .with_ssid(wifi_ssid)
            .with_password(wifi_password.into()),
    );
    wifi_controller
        .set_config(&wifi_config)
        .expect("Failed to configure Wi-Fi station");

    let _connector = BleConnector::new(peripherals.BT, Default::default());

    let mut led = Output::new(peripherals.GPIO22, Level::Low, Default::default());

    loop {
        info!("Hello world!");
        led.toggle();
        let delay_start = Instant::now();
        while delay_start.elapsed() < Duration::from_millis(500) {}
    }

    // for inspiration have a look at the examples at https://github.com/esp-rs/esp-hal/tree/esp-hal-v1.1.0/examples
}
