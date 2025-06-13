# Specify the minimum version of CMake required
cmake_minimum_required(VERSION 3.10)

# Define the project name
project(ajazz_time_correction_tool)

# Add an executable target named 'my_first_app' from the main.cpp source file
add_executable(ajazz_time_correction_tool main.cpp)

# --- HIDAPI Linking ---
# Option to control hidapi usage (useful for CI where paths are provided via env vars)
# For local builds, find_package will be attempted if env vars are not set.
# Default to OFF, meaning we expect hidapi to be provided externally (either via env vars or find_package)
option(USE_BUNDLED_HIDAPI "Set to ON to use a bundled hidapi (if one were present), OFF to find external." OFF)

if(USE_BUNDLED_HIDAPI)
    # This block would be used if we ever re-introduce a bundled version.
    # For now, it means if USE_BUNDLED_HIDAPI is ON, it won't link hidapi.
    message(WARNING "USE_BUNDLED_HIDAPI is ON. HIDAPI will not be linked. Set to OFF for external linking.")
    # If you had a dummy/stub hidapi.h for when it's not linked, you might include its path here.
    # target_include_directories(ajazz_time_correction_tool PRIVATE some_dummy_hidapi_include_path)
else()
    # Attempt to use environment variables first (typically set by CI)
    if(DEFINED ENV{HIDAPI_INCLUDE_DIR} AND DEFINED ENV{HIDAPI_LIBRARY_DIR})
        message(STATUS "Using HIDAPI from environment variables:")
        message(STATUS "  HIDAPI_INCLUDE_DIR: $ENV{HIDAPI_INCLUDE_DIR}")
        message(STATUS "  HIDAPI_LIBRARY_DIR: $ENV{HIDAPI_LIBRARY_DIR}")
        target_include_directories(ajazz_time_correction_tool PRIVATE "$ENV{HIDAPI_INCLUDE_DIR}")

        # Construct library path. Note: hidapi.lib for MSVC, libhidapi.so for Linux, libhidapi.dylib for macOS
        if(MSVC) # MSVC is a variable CMake sets if using Microsoft Visual C++ compiler
            set(HIDAPI_LIBRARY_FILE "$ENV{HIDAPI_LIBRARY_DIR}/hidapi.lib")
        elseif(UNIX AND NOT APPLE) # Linux
            set(HIDAPI_LIBRARY_FILE "$ENV{HIDAPI_LIBRARY_DIR}/libhidapi-hidraw.so") # Or just libhidapi.so depending on build
        elseif(APPLE) # macOS
            set(HIDAPI_LIBRARY_FILE "$ENV{HIDAPI_LIBRARY_DIR}/libhidapi.dylib")
        else()
            message(WARNING "Unsupported platform for HIDAPI library linking via environment variables. Assuming .lib or .so")
            set(HIDAPI_LIBRARY_FILE "$ENV{HIDAPI_LIBRARY_DIR}/hidapi") # Generic fallback, might fail
        endif()

        if(EXISTS "${HIDAPI_LIBRARY_FILE}")
            target_link_libraries(ajazz_time_correction_tool PRIVATE "${HIDAPI_LIBRARY_FILE}")
            message(STATUS "Linking against HIDAPI library: ${HIDAPI_LIBRARY_FILE}")
        else()
            message(FATAL_ERROR "HIDAPI library not found at specified HIDAPI_LIBRARY_DIR: ${HIDAPI_LIBRARY_FILE}. Ensure the correct library file name and path are provided.")
        endif()
    else()
        # Fallback to find_package for local builds or if env vars not set
        message(STATUS "Environment variables HIDAPI_INCLUDE_DIR or HIDAPI_LIBRARY_DIR not set. Attempting find_package(HIDAPI).")
        find_package(HIDAPI REQUIRED)
        if(HIDAPI_FOUND)
            message(STATUS "Found HIDAPI via find_package.")
            message(STATUS "  HIDAPI Libraries: ${HIDAPI_LIBRARIES}")
            message(STATUS "  HIDAPI Include Dirs: ${HIDAPI_INCLUDE_DIRS}")
            target_include_directories(ajazz_time_correction_tool PRIVATE ${HIDAPI_INCLUDE_DIRS})
            target_link_libraries(ajazz_time_correction_tool PRIVATE ${HIDAPI_LIBRARIES})
        else()
            message(FATAL_ERROR "HIDAPI not found via find_package. Please install HIDAPI development package or set HIDAPI_INCLUDE_DIR and HIDAPI_LIBRARY_DIR environment variables.")
        endif()
    endif()
endif()
# --- End HIDAPI Linking ---