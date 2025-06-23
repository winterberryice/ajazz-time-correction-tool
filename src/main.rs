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

/// This version finds ALL matching device interfaces and tries to send the command to each one.
fn run_app() -> Result<(), Box<dyn Error>> {
    let api = HidApi::new()?;

    // 1. Find ALL interfaces that match the VID/PID
    let device_interfaces: Vec<_> = api
        .device_list()
        .filter(|d| d.vendor_id() == TARGET_VID && d.product_id() == TARGET_PID)
        .collect();

    if device_interfaces.is_empty() {
        return Err(format!(
            "No device found with VID={:04x}, PID={:04x}",
            TARGET_VID, TARGET_PID
        ).into());
    }

    println!("Found {} matching device interface(s). Testing each one...", device_interfaces.len());
    let mut success = false;

    // 2. Loop through each found interface
    for device_info in device_interfaces {
        println!("\n--- Testing Interface #{} ---", device_info.interface_number());
        
        match device_info.open_device(&api) {
            Ok(device) => {
                // 3. Try to send the command
                if sync_time(&device).is_ok() {
                    success = true; // Mark that at least one command sent successfully
                }
            }
            Err(e) => eprintln!("  Could not open interface: {}", e),
        }
    }

    if !success {
         return Err("Sent command to all interfaces, but none succeeded.".into());
    }

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
