use chrono::prelude::*;
use hidapi::{HidApi, HidDevice};
use std::error::Error;

// --- Constants ---
const AJAZZ_VID: u16 = 0x1E7D;

// --- Main Application Entry Point ---

fn main() {
    println!("Attempting to sync time with Ajazz keyboard...");

    // Run the core application logic and print any error that bubbles up.
    if let Err(e) = run_app() {
        eprintln!("\nAn error occurred: {}", e);
    }
}

/// Contains the main application flow, returning a Result for robust error handling.
fn run_app() -> Result<(), Box<dyn Error>> {
    let api = HidApi::new()?;

    // 1. Find the Keyboard
    // Find the first device that matches the Ajazz Vendor ID.
    // .ok_or_else() converts the Option into a Result, providing a nice error message if None.
    let device_info = api
        .device_list()
        .find(|d| d.vendor_id() == AJAZZ_VID)
        .ok_or_else(|| format!("No compatible Ajazz keyboard found (VID: {:04x})", AJAZZ_VID))?;

    println!(
        "Found device: {} (VID: {:04x}, PID: {:04x})",
        device_info.product_string().unwrap_or("Unknown"),
        device_info.vendor_id(),
        device_info.product_id()
    );

    // 2. Open the device
    // .map_err() allows us to add context to the error message.
    let device = device_info.open_device(&api).map_err(|e| {
        format!(
            "Failed to open device: {}. On Linux, try running with 'sudo'.",
            e
        )
    })?;

    // 3. Create and send the command
    sync_time(&device)?;

    Ok(())
}

/// Creates the time-sync payload and sends it to the provided device.
fn sync_time(device: &HidDevice) -> Result<(), Box<dyn Error>> {
    println!("Preparing to send time-sync command...");

    // Create a 65-byte buffer, initialized to all zeros
    let mut payload = vec![0u8; 65];
    let now = Local::now();

    // This is the "secret recipe" we discovered
    payload[0] = 0x00; // Report ID
    payload[1] = 0x01; // Command Prefix
    payload[2] = 0x5A; // Command ID for "Set Time"

    payload[3] = (now.year() % 100) as u8;
    payload[4] = now.month() as u8;
    payload[5] = now.day() as u8;
    payload[6] = now.hour() as u8;
    payload[7] = now.minute() as u8;
    payload[8] = now.second() as u8;

    // Magic number/checksum at the end
    payload[63] = 0xAA;
    payload[64] = 0x55;

    // Send the command and handle potential errors
    device.send_feature_report(&payload)?;

    println!("\nSUCCESS: The time-sync command was sent to the keyboard!");

    Ok(())
}
