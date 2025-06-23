use chrono::prelude::*;
use hidapi::{HidApi, HidDevice};
use std::error::Error;

// --- CORRECTED Constants ---
const TARGET_VID: u16 = 0x0c45;
const TARGET_PID: u16 = 0x8009;
const TARGET_INTERFACE: i32 = 3;

// --- Main Application Entry Point ---

fn main() {
    println!("Attempting to sync time with Ajazz keyboard...");
    if let Err(e) = run_app() {
        eprintln!("\nAn error occurred: {}", e);
    }
}

/// Contains the main application flow, with corrected device finding logic.
fn run_app() -> Result<(), Box<dyn Error>> {
    let api = HidApi::new()?;

    // 1. Find the Keyboard using the CORRECT, specific identifiers
    // The key change is here. We now match VID, PID, and the Interface Number.
    let device_info = api
        .device_list()
        .find(|d| {
            d.vendor_id() == TARGET_VID
                && d.product_id() == TARGET_PID
                && d.interface_number() == TARGET_INTERFACE
        })
        .ok_or_else(|| {
            format!(
                "Could not find target device (VID={:04x}, PID={:04x}, Interface={})",
                TARGET_VID, TARGET_PID, TARGET_INTERFACE
            )
        })?;

    println!(
        "Found target device interface: {} (VID: {:04x}, PID: {:04x})",
        device_info.product_string().unwrap_or("Unknown"),
        device_info.vendor_id(),
        device_info.product_id()
    );

    // 2. Open the device
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
/// THIS VERSION IS MODIFIED FOR TESTING: It always sends the time 12:12:00.
fn sync_time(device: &HidDevice) -> Result<(), Box<dyn Error>> {
    println!("Preparing to send test command (time: 12:12:00)...");

    // Create a 65-byte buffer, initialized to all zeros
    let mut payload = vec![0u8; 65];
    let now = Local::now(); // We still use this to get the correct date

    // This is the "secret recipe" we discovered
    payload[0] = 0x00; // Report ID
    payload[1] = 0x01; // Command Prefix
    payload[2] = 0x5A; // Command ID for "Set Time"

    // --- Date (from current system time) ---
    payload[3] = (now.year() % 100) as u8;
    payload[4] = now.month() as u8;
    payload[5] = now.day() as u8;

    // --- Time (HARDCODED FOR TESTING) ---
    payload[6] = 12; // Hour
    payload[7] = 12; // Minute
    payload[8] = 00; // Second

    // Magic number/checksum at the end
    payload[63] = 0xAA;
    payload[64] = 0x55;

    // Send the command and handle potential errors
    device.send_feature_report(&payload)?;

    println!("\nSUCCESS: The test command was sent to the keyboard!");

    Ok(())
}
