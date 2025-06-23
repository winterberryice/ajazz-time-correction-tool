use hidapi::HidApi;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    println!("--- HID Device Diagnostic ---");
    println!("Listing all available HID devices to find the correct interface...\n");

    let api = HidApi::new()?;
    for device in api.device_list() {
        println!("=============================================");
        println!(
            "VID: {:04x} | PID: {:04x} | Interface #: {}",
            device.vendor_id(),
            device.product_id(),
            device.interface_number()
        );
        println!("  - Product:      {}", device.product_string().unwrap_or("N/A"));
        println!("  - Manufacturer: {}", device.manufacturer_string().unwrap_or("N/A"));
        println!("  - Path:         {}", device.path().to_string_lossy());
        println!(
            "  - Usage Page:   0x{:04x} | Usage: 0x{:04x}",
            device.usage_page(),
            device.usage()
        );
    }
    println!("=============================================");
    println!("\nDiagnostic complete.");
    Ok(())
}
