use anyhow::*;
use esp_idf_hal::{prelude::Peripherals, reset::WakeupReason};
use esp_idf_sys::{self as _, esp}; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported
use log::*;

mod ulp_code_vars {
    include!(env!("ULP_FSM_RS"));
}

const ULP_CODE: &[u8] = include_bytes!(env!("ULP_FSM_BIN"));

fn main() -> Result<()> {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_sys::link_patches();
    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    let wakeup_reason = WakeupReason::get();
    info!("Wakeup reason: {:?}", wakeup_reason);

    info!("Hello, world!");

    let peripherals = Peripherals::take().unwrap();

    let mut ulp_driver = esp_idf_hal::ulp::UlpDriver::new(peripherals.ulp)?;

    info!("Feeling sleepy");
    unsafe {
        ulp_driver.load(ULP_CODE)?;
        ulp_driver.start(ulp_code_vars::wake_up)?;

        esp!(esp_idf_sys::esp_sleep_enable_ulp_wakeup())?;
        esp_idf_sys::esp_deep_sleep_start();
    };
}
