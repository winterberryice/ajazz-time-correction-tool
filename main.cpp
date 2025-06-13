#include <iostream>
#include <string>
#include <algorithm>
#include <cwchar> // For mbstowcs

// It's good practice to place system headers or C-library headers
// before C++ standard library headers if there's a mix.
// However, hidapi.h is a project-local C header.
extern "C" {
#include "hidapi/hidapi.h"
}

// Helper function to convert wchar_t* to std::string
std::string wchar_to_string(const wchar_t* wstr) {
    if (!wstr) {
        return "";
    }
    std::mbstate_t state = std::mbstate_t();
    // Call wcsrtombs with null destination to get length
    const wchar_t* wstr_ptr = wstr; // wcsrtombs might modify the pointer
    std::size_t len = std::wcsrtombs(nullptr, &wstr_ptr, 0, &state);
    if (static_cast<std::size_t>(-1) == len) {
        return ""; // Or throw an exception
    }
    std::string str(len, '\0');
    // Reset state and pointer for actual conversion
    state = std::mbstate_t();
    wstr_ptr = wstr;
    std::wcsrtombs(&str[0], &wstr_ptr, len, &state);
    return str;
}

int main() {
    // Initialize the HIDAPI library
    if (hid_init()) {
        std::cerr << "Error: Failed to initialize HIDAPI." << std::endl;
        return 1;
    }

    std::cout << "Scanning for Ajazz keyboards..." << std::endl;

    // Enumerate all HID devices
    struct hid_device_info *devs, *cur_dev;
    devs = hid_enumerate(0x0, 0x0); // Vendor ID and Product ID 0x0 means enumerate all devices
    cur_dev = devs;

    bool found_ajazz = false;

    while (cur_dev) {
        if (cur_dev->product_string) {
            std::string product_name_str = wchar_to_string(cur_dev->product_string);
            std::string product_name_lower = product_name_str;
            std::transform(product_name_lower.begin(), product_name_lower.end(), product_name_lower.begin(),
                           [](unsigned char c){ return std::tolower(c); });

            if (product_name_lower.find("ajazz") != std::string::npos) {
                found_ajazz = true;
                std::cout << "Found Ajazz Device:" << std::endl;
                if (cur_dev->path) {
                    std::cout << "  Path: " << cur_dev->path << std::endl;
                } else {
                    std::cout << "  Path: (null)" << std::endl;
                }
                std::cout << "  Vendor ID: 0x" << std::hex << cur_dev->vendor_id << std::endl;
                std::cout << "  Product ID: 0x" << std::hex << cur_dev->product_id << std::endl;
                std::cout << "  Product String: " << product_name_str << std::dec << std::endl; // Switch back to decimal for other numbers
            }
        }
        cur_dev = cur_dev->next;
    }

    if (!found_ajazz) {
        std::cout << "No Ajazz keyboards found." << std::endl;
    }

    // Free the enumerated device list
    if (devs) {
        hid_free_enumeration(devs);
    }

    // Finalize the HIDAPI library
    if (hid_exit()) {
        std::cerr << "Error: Failed to finalize HIDAPI." << std::endl;
        // Depending on the application, you might still want to return 0 if the main logic succeeded
        // or return a specific error code.
        return 1;
    }

    std::cout << "Scan complete." << std::endl;
    return 0;
}