const std = @import("std");
const c = @cImport({
    @cInclude("hidapi/hidapi.h");
});

// Helper function to print a wchar_t C string.
fn printWcharString(writer: std.io.AnyWriter, prefix: []const u8, wstr: ?[*]const c.wchar_t, allocator: std.mem.Allocator) !void {
    if (wstr == null) {
        try writer.print("{s} (null)\n", .{prefix});
        return;
    }

    var temp_buf = std.ArrayList(u8).init(allocator);
    defer temp_buf.deinit();

    var i: usize = 0;
    while (wstr.?[i] != 0) : (i += 1) {
        const wide_char = wstr.?[i];
        if (wide_char > 0 and wide_char <= 127) { // Printable ASCII
            try temp_buf.append(@intCast(u8, wide_char));
        } else {
            try temp_buf.append('?'); // Placeholder
        }
    }

    if (temp_buf.items.len == 0) {
        try writer.print("{s} (empty or non-printable)\n", .{prefix});
    } else {
        try writer.print("{s} {s}\n", .{prefix, temp_buf.items});
    }
}

pub fn main() !void {
    const stdout_writer = std.io.getStdOut().writer();
    const allocator = std.heap.page_allocator;

    if (c.hid_init() != 0) {
        try stdout_writer.print("Failed to initialize HIDAPI\n", .{}); // Escaped newline
        std.process.exit(1);
    }
    defer {
        _ = c.hid_exit();
    }

    var enumerated_devices = c.hid_enumerate(0x0, 0x0);
    if (enumerated_devices == null) {
        try stdout_writer.print("No HID devices found, or an error occurred during enumeration.\n", .{}); // Escaped newline
        return;
    }
    defer c.hid_free_enumeration(enumerated_devices);

    var current_device_node = enumerated_devices;
    var device_counter: u32 = 0;

    try stdout_writer.print("Scanning for HID devices...\n\n", .{}); // Escaped newlines

    while (current_device_node != null) : (current_device_node = current_device_node.?.next) {
        device_counter += 1;
        try stdout_writer.print("Device {d}:\n", .{device_counter}); // Escaped newline

        if (current_device_node.?.path != null) {
            try stdout_writer.print("  Path: {s}\n", .{std.mem.span(current_device_node.?.path)}); // Escaped newline
        } else {
            try stdout_writer.print("  Path: (null)\n", .{}); // Escaped newline
        }

        try stdout_writer.print("  Vendor ID: 0x{x:04}\n", .{current_device_node.?.vendor_id}); // Escaped newline
        try stdout_writer.print("  Product ID: 0x{x:04}\n", .{current_device_node.?.product_id}); // Escaped newline

        try printWcharString(stdout_writer, "  Manufacturer:", current_device_node.?.manufacturer_string, allocator);
        try printWcharString(stdout_writer, "  Product:", current_device_node.?.product_string, allocator);

        try stdout_writer.print("\n", .{}); // Escaped newline
    }

    if (device_counter == 0) {
        try stdout_writer.print("No HID devices were ultimately processed from the list.\n", .{}); // Escaped newline
    } else {
        try stdout_writer.print("Finished enumerating devices. Total found: {d}\n", .{device_counter}); // Escaped newline
    }
}
