const std = @import("std");

// Regarding HIDAPI Source Code:
//
// CI Builds (GitHub Actions):
//   The hidapi source code is automatically cloned from https://github.com/libusb/hidapi.git
//   into the `libs/hidapi` directory during the CI workflow. No manual action is needed for CI.
//
// Local Builds:
//   For local development and building, you need to ensure the hidapi source code is present
//   in the `libs/hidapi` directory. If it's not already there (e.g., if you cloned this
//   project without hidapi), you can clone it yourself:
//
//   git clone https://github.com/libusb/hidapi.git libs/hidapi
//
//   Alternatively, you can download a release tarball/zip from the hidapi repository and
//   extract its contents into `libs/hidapi`.
//
// Expected Directory Structure:
//   The build system expects the hidapi source files to be structured as follows (relative
//   to the project root):
//     libs/hidapi/hidapi/hidapi.h
//     libs/hidapi/hidapi/hid.c
//     libs/hidapi/linux/hid.c   (for Linux)
//     libs/hidapi/windows/hid.c (for Windows)
//     libs/hidapi/mac/hid.c     (for macOS)
//   Ensure this structure is maintained if you manually place the files.
//

pub fn build(b: *std.Build) void {
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});

    const exe = b.addExecutable(.{
        .name = "app", // Single executable named "app"
        .root_source_file = b.path("src/main.zig"),
        .target = target,
        .optimize = optimize,
    });

    // Add hidapi sources and include directory
    exe.addIncludePath(b.path("libs/hidapi/hidapi"));
    exe.addCSourceFile(.{ .file = b.path("libs/hidapi/hidapi/hid.c"), .flags = &.{"-Wno-implicit-function-declaration"} });

    // Add platform-specific sources and libraries
    switch (target.result.os.tag) {
        .linux => {
            exe.addCSourceFile(.{ .file = b.path("libs/hidapi/linux/hid.c"), .flags = &.{"-Wno-implicit-function-declaration"} });
            exe.linkSystemLibrary("udev");
            exe.linkSystemLibrary("usb-1.0");
        },
        .macos => {
            exe.addCSourceFile(.{ .file = b.path("libs/hidapi/mac/hid.c"), .flags = &.{"-Wno-implicit-function-declaration"} });
            exe.linkFramework("IOKit");
            exe.linkFramework("CoreFoundation");
        },
        .windows => {
            exe.addCSourceFile(.{ .file = b.path("libs/hidapi/windows/hid.c"), .flags = &.{"-Wno-implicit-function-declaration"} });
            // SetupAPI is typically linked by default on Windows
        },
        else => {
            @panic("Unsupported OS for hidapi");
        },
    }

    // Prefer static linking for hidapi
    exe.linkLibC(); // Ensure libc is linked
    // Note: True static linking of hidapi might require compiling hidapi as a static library first
    // and then linking it. For simplicity, we're adding source files directly.

    b.installArtifact(exe);

    // Optional: Define a step to run the executable
    const run_cmd = b.addRunArtifact(exe);
    run_cmd.step.dependOn(b.getInstallStep());
    if (b.args) |args| {
        run_cmd.addArgs(args);
    }
    const run_step = b.step("run", "Run the app");
    run_step.dependOn(&run_cmd.step);
}
