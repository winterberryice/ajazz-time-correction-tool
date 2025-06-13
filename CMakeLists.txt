# Specify the minimum version of CMake required
cmake_minimum_required(VERSION 3.10)

# Define the project name
project(ajazz_time_correction_tool)

add_subdirectory(libs/hidapi)

# Add an executable target named 'my_first_app' from the main.cpp source file
add_executable(ajazz_time_correction_tool main.cpp)
target_link_libraries(ajazz_time_correction_tool PRIVATE hidapi_lib)
target_include_directories(ajazz_time_correction_tool PRIVATE libs/hidapi)