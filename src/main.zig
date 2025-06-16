const std = @import("std");

// C imports for hidapi
// This assumes hidapi.h is available via the include path "-Ilibs/hidapi/hidapi"
// added in build.zig.
const c = @cImport({
    @cInclude("hidapi.h");
});

// Helper function to convert C wchar_t* (wide string) to Zig []const u8
fn wcharToUtf8(allocator: std.mem.Allocator, wide_str: ?[*]const c.wchar_t) ![]const u8 {
    if (wide_str == null) return "";
    var len: usize = 0;
    while (wide_str.?[len] != 0) : (len += 1) {}
    return std.unicode.utf16leToUtf8Alloc(allocator, std.mem.sliceTo(wide_str.?, len));
}

pub fn main() !void {
    const allocator = std.heap.page_allocator;
    const stderr = std.io.getStdErr().writer();

    // Initialize hidapi
    if (c.hid_init() != 0) {
        try stderr.print("Failed to initialize HIDAPI\n", .{});
        return;
    }

    // Enumerate all HID devices
    var devices: ?*c.hid_device_info = c.hid_enumerate(0x0, 0x0);
    if (devices == null) {
        try stderr.print("Failed to enumerate HID devices or no devices found.\n", .{});
        _ = c.hid_exit();
        return;
    }

    var current_device = devices;
    var found_keyboards = false;

    std.debug.print("Scanning for keyboard devices...\n", .{});

    // Iterate through the device list
    while (current_device != null) : (current_device = current_device.?.*.next) {
        const dev = current_device.?.*;

        // Check for Keyboard usage page (0x01) and usage ID (0x06)
        // These values are standard for keyboards.
        if (dev.usage_page == 0x01 and dev.usage == 0x06) {
            found_keyboards = true;
            std.debug.print("----------------------------------------\n", .{});
            std.debug.print("Keyboard Found:\n", .{});
            std.debug.print("  Vendor ID      : {x:04X}\n", .{dev.vendor_id});
            std.debug.print("  Product ID     : {x:04X}\n", .{dev.product_id});

            const manufacturer_string = try wcharToUtf8(allocator, dev.manufacturer_string);
            defer if (manufacturer_string.len > 0) allocator.free(manufacturer_string);
            std.debug.print("  Manufacturer   : {s}\n", .{if (manufacturer_string.len > 0) manufacturer_string else "N/A"});

            const product_string = try wcharToUtf8(allocator, dev.product_string);
            defer if (product_string.len > 0) allocator.free(product_string);
            std.debug.print("  Product        : {s}\n", .{if (product_string.len > 0) product_string else "N/A"});

            // Path can be null if not available or on certain platforms/devices
            if (dev.path) |path_ptr| {
                 const path_slice = std.mem.span(std.mem.sliceTo(path_ptr, std.mem.strlen(path_ptr)));
                 std.debug.print("  Path           : {s}\n", .{path_slice});
            } else {
                 std.debug.print("  Path           : N/A\n", .{});
            }
            std.debug.print("  Usage Page     : {x:04X}\n", .{dev.usage_page});
            std.debug.print("  Usage          : {x:04X}\n", .{dev.usage});
            std.debug.print("----------------------------------------\n", .{});
        }
    }

    // Free the enumerated device list
    c.hid_free_enumeration(devices);

    if (!found_keyboards) {
        std.debug.print("No keyboard devices found.\n", .{});
    }

    // Finalize hidapi
    if (c.hid_exit() != 0) {
        try stderr.print("Failed to finalize HIDAPI\n", .{});
        // continue anyway, as we are exiting
    }
}
