use hidapi::{HidApi, HidDevice};
use chrono::prelude::*;
use std::error::Error;
use std::thread;
use std::time::Duration;

// --- Confirmed Constants ---
const TARGET_VID: u16 = 0x0c45;
const TARGET_PID: u16 = 0x8009;
const TARGET_INTERFACE: i32 = 3;

// --- Main Application Entry Point ---
fn main() {
    println!("Attempting full 16-packet time-sync handshake...");
    if let Err(e) = run_app() {
        eprintln!("\nAn error occurred: {}", e);
    }
}

/// Finds the correct device interface and runs the full command sequence.
fn run_app() -> Result<(), Box<dyn Error>> {
    let api = HidApi::new()?;
    let device_info = api.device_list()
        .find(|d| d.vendor_id() == TARGET_VID && d.product_id() == TARGET_PID && d.interface_number() == TARGET_INTERFACE)
        .ok_or_else(|| "Target device interface not found.")?;

    println!("Found target device interface: {}", device_info.product_string().unwrap_or("Unknown"));
    let device = device_info.open_device(&api)?;

    // Perform the full handshake
    perform_full_handshake(&device)?;

    Ok(())
}

// --- Helper functions for sending/receiving reports ---

/// Sends a SET Feature Report command to the device.
fn send_feature_report(device: &HidDevice, command_name: &str, data_fragment: &[u8]) -> Result<(), Box<dyn Error>> {
    println!("  - SET: {}", command_name);
    let mut buffer = vec![0x00; data_fragment.len() + 1];
    buffer[0] = 0x00; // Report ID 0
    buffer[1..].copy_from_slice(data_fragment);
    device.send_feature_report(&buffer)?;
    Ok(())
}

/// Performs a GET Feature Report request.
fn get_feature_report(device: &HidDevice, command_name: &str) -> Result<(), Box<dyn Error>> {
    println!("  - GET: {}", command_name);
    let mut read_buffer = [0u8; 65];
    read_buffer[0] = 0x00; // Request Report ID 0
    device.get_feature_report(&mut read_buffer)?;
    Ok(())
}


/// Performs the full sequence of SET and GET commands.
fn perform_full_handshake(device: &HidDevice) -> Result<(), Box<dyn Error>> {
    println!("\nPerforming 4-stage handshake...");
    let short_delay = Duration::from_millis(30);

    // --- Stage 1 ---
    let cmd1 = [0x04, 0x18, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    send_feature_report(device, "Handshake Step 1", &cmd1)?;
    get_feature_report(device, "Handshake Step 1 Response")?;
    thread::sleep(short_delay);

    // --- Stage 2 ---
    let cmd2 = [0x04, 0x28, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    send_feature_report(device, "Handshake Step 2", &cmd2)?;
    get_feature_report(device, "Handshake Step 2 Response")?;
    thread::sleep(short_delay);

    // --- Stage 3: The actual Time Sync command with MOCK time ---
    let now = Local::now(); // Still used for the date
    let mut cmd3 = [0u8; 64];
    cmd3[0] = 0x00; cmd3[1] = 0x01; cmd3[2] = 0x5A;
    
    // Use real date
    cmd3[3] = (now.year() % 100) as u8;
    cmd3[4] = now.month() as u8;
    cmd3[5] = now.day() as u8;

    // Use mock time for testing
    println!("  - Using mock time: 13:37:42");
    cmd3[6] = 13; // Mock Hour
    cmd3[7] = 37; // Mock Minute
    cmd3[8] = 42; // Mock Second

    cmd3[62] = 0xAA; cmd3[63] = 0x55;
    send_feature_report(device, "Time Sync Data", &cmd3)?;
    get_feature_report(device, "Time Sync Data Response")?;
    thread::sleep(short_delay);
    
    // --- Stage 4 ---
    let cmd4 = [0x04, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    send_feature_report(device, "Handshake Step 4", &cmd4)?;
    get_feature_report(device, "Handshake Step 4 Response")?;

    println!("\nSUCCESS: The full 16-packet handshake sequence was sent!");
    
    Ok(())
}
