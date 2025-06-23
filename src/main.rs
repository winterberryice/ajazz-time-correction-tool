use chrono::prelude::*;
use hidapi::{HidApi, HidDevice};
use std::error::Error;

// --- Confirmed Constants ---
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

/// Final version: Finds the specific, correct interface.
fn run_app() -> Result<(), Box<dyn Error>> {
    let api = HidApi::new()?;

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
    
    let device = device_info.open_device(&api)?;
    
    // Call the sync function which now uses the correct command type and payload length
    sync_time(&device)?;

    Ok(())
}

/// Final version: Uses `device.write()` with a 64-byte payload.
fn sync_time(device: &HidDevice) -> Result<(), Box<dyn Error>> {
    println!("Sending command with device.write() and 64-byte payload...");

    let payload: [u8; 64] = [
        0x00, 0x01, 0x5A, 0x19, 0x06, 0x0B, 0x09, 0x24, 0x3A, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xAA, 0x55
    ];

    // --- The Critical Change ---
    // We use `device.write()` and pass a 64-byte slice that skips the leading Report ID.
    // This exactly matches the length and likely command type from the capture.
    //device.write(&payload[..])?;
    device.send_feature_report(&payload[..])?;

    println!("\nSUCCESS: The 64-byte write() command was sent to the keyboard!");
    println!("Please check if the keyboard's time is now set to June 11, 2025, 09:36.");

    Ok(())
}
