const std = @import("std");

pub fn build(b: *std.Build) void {
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});

    const exe = b.addExecutable(.{
        .name = "app", // Single executable named "app"
        .root_source_file = b.path("src/main.zig"),
        .target = target,
        .optimize = optimize,
    });

    // --- START HIDAPI INTEGRATION ---
    // Assuming hidapi is installed in "hidapi_install" at the root of the project in the GHA environment
    // $GITHUB_WORKSPACE/hidapi_install
    // addIncludePath needs path to the directory containing 'hidapi.h' or the 'hidapi' directory itself.
    // If hidapi.h is at 'hidapi_install/include/hidapi/hidapi.h', then use:
    exe.addIncludePath(b.path("hidapi_install/include/hidapi"));
    // If hidapi.h is at 'hidapi_install/include/hidapi.h', then use:
    // exe.addIncludePath(b.path("hidapi_install/include"));

    // Add library path for libhidapi-hidraw.a
    exe.addLibraryPath(b.path("hidapi_install/lib"));

    // Link against the static hidapi-hidraw library
    //addObjectFile expects a path to the .a or .o file
    exe.addObjectFile(b.path("hidapi_install/lib/libhidapi-hidraw.a"));

    // Link against udev and usb-1.0, which hidapi depends on.
    // These are system libraries, so linkSystemLibrary is appropriate.
    // This is only needed for Linux.
    if (exe.target.isLinux()) {
        exe.linkSystemLibrary("udev");
        exe.linkSystemLibrary("usb-1.0");
    }
    // --- END HIDAPI INTEGRATION ---

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
