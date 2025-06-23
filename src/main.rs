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

/// Creates and sends a hardcoded, known-good payload for testing.
fn sync_time(device: &HidDevice) -> Result<(), Box<dyn Error>> {
    println!("Sending a hardcoded, known-good command from the original capture...");

    // This is a byte-for-byte copy of the successful Wireshark capture.
    // The payload sets the time to June 11, 2025, 09:36:58.
    let payload: [u8; 65] = [
        0x00, 0x01, 0x5A, 0x19, 0x06, 0x0B, 0x09, 0x24, 0x3A, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xAA,
        0x55
    ];

    // Send the command and handle potential errors.
    // Note we pass it as a slice `&payload[..]`
    device.send_feature_report(&payload[..])?;

    println!("\nSUCCESS: The known-good command was sent to the keyboard!");
    println!("Please check if the keyboard's time is now set to June 11, 2025, 09:36.");

    Ok(())
}
