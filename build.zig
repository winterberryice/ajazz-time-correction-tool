const std = @import("std");

pub fn build(b: *std.Build) void {
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});

    const exe1 = b.addExecutable(.{
        .name = "hello",
        .root_source_file = .{ .path = "src/main.zig" },
        .target = target,
        .optimize = optimize,
    });

    b.installArtifact(exe1);

    const exe2 = b.addExecutable(.{
        .name = "hello2",
        .root_source_file = .{ .path = "src/main.zig" }, // Using the same source for simplicity
        .target = target,
        .optimize = optimize,
    });

    b.installArtifact(exe2);

    // Optional: Define a step to run the first executable
    const run_cmd1 = b.addRunArtifact(exe1);
    run_cmd1.step.dependOn(b.getInstallStep());
    if (b.args) |args| {
        run_cmd1.addArgs(args);
    }
    const run_step1 = b.step("run", "Run the 'hello' app");
    run_step1.dependOn(&run_cmd1.step);

    // Optional: Define a step to run the second executable (if needed for testing)
    // const run_cmd2 = b.addRunArtifact(exe2);
    // run_cmd2.step.dependOn(b.getInstallStep());
    // if (b.args) |args| {
    //     run_cmd2.addArgs(args);
    // }
    // const run_step2 = b.step("run-hello2", "Run the 'hello2' app");
    // run_step2.dependOn(&run_cmd2.step);
}
