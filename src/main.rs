use hidapi::HidApi;

fn print_hid_devices(api: &HidApi) {
    for device in api.device_list() {
        println!("  VID: {:04x}, PID: {:04x}", device.vendor_id(), device.product_id());

        println!("    Path:           {}", device.path().to_string_lossy());
        println!("    Interface #:    {}", device.interface_number());
        println!("    Usage Page:     0x{:04x}", device.usage_page());

        // CORRECTED: The method is .usage() not .usage_id()
        println!("    Usage ID:       0x{:04x}", device.usage());

        println!("    Product:        {}", device.product_string().unwrap_or("N/A"));
        println!("    Manufacturer:   {}", device.manufacturer_string().unwrap_or("N/A"));
        println!("    Serial:         {}", device.serial_number().unwrap_or("N/A"));
        println!();
    }
}

fn run_app() -> Result<(), String> {
    println!("Printing all available HID devices:");
    match HidApi::new() {
        Ok(api) => {
            print_hid_devices(&api);
            Ok(())
        }
        Err(e) => {
            Err(format!("Error initializing HidApi: {}", e))
        }
    }
}

fn main() {
    if let Err(e) = run_app() {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
