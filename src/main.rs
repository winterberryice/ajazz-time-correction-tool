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

/// This version tests the "Double Zero" hypothesis, where the data payload
/// itself starts with 0x00.
fn sync_time(device: &HidDevice) -> Result<(), Box<dyn Error>> {
    println!("Testing command with a 'double zero' payload structure...");

    // This is the new, corrected payload based on your observation.
    // The buffer is 65 bytes long.
    let payload: [u8; 65] = [
        // The FIRST zero is the Report ID, consumed by hidapi
        0x00, 
        
        // The SECOND zero is the first byte of the 64-byte data fragment
        0x00, 
        
        // The rest of the known-good command
        0x01, 0x5A, 0x19, 0x06, 0x0B, 0x09, 0x24, 0x3A, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xAA, 0x55
    ];

    // We send the entire 65-byte buffer. hidapi will use payload[0] as the Report ID
    // and send the remaining 64 bytes (which now correctly start with 0x00) as the data.
    device.send_feature_report(&payload)?;

    println!("\nSUCCESS: The corrected 'double zero' command was sent!");
    println!("Please check the keyboard's time.");

    Ok(())
}
