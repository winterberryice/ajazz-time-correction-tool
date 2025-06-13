# Specify the minimum version of CMake required
cmake_minimum_required(VERSION 3.10)

# Define the project name
project(ajazz_time_correction_tool)

# Add an executable target named 'my_first_app' from the main.cpp source file
add_executable(ajazz_time_correction_tool main.cpp)

# --- HIDAPI Linking ---
# Check if HIDAPI paths are explicitly provided as CMake variables (e.g., by CI via -D options)
if(HIDAPI_INCLUDE_DIR AND HIDAPI_LIBRARY_DIR)
    message(STATUS "Using HIDAPI from CMake variables:")
    message(STATUS "  HIDAPI_INCLUDE_DIR: ${HIDAPI_INCLUDE_DIR}")
    message(STATUS "  HIDAPI_LIBRARY_DIR: ${HIDAPI_LIBRARY_DIR}")
    target_include_directories(ajazz_time_correction_tool PRIVATE "${HIDAPI_INCLUDE_DIR}")

    # Construct library path.
    if(WIN32) # WIN32 is a CMake variable that is TRUE on Windows (MSVC, MinGW, Clang for Windows)
        set(HIDAPI_LIBRARY_FILE "${HIDAPI_LIBRARY_DIR}/hidapi.lib")
    elseif(UNIX AND NOT APPLE) # Linux
        set(HIDAPI_LIBRARY_FILE "${HIDAPI_LIBRARY_DIR}/libhidapi-hidraw.so") # Common name for hidraw backend
    elseif(APPLE) # macOS
        set(HIDAPI_LIBRARY_FILE "${HIDAPI_LIBRARY_DIR}/libhidapi.dylib")
    else()
        message(WARNING "Target platform not explicitly supported for HIDAPI library path construction via CMake variables. Assuming a common name like hidapi.lib or libhidapi.so.")
        # Attempt a generic name; this might need adjustment based on the actual library name on other platforms
        set(HIDAPI_LIBRARY_FILE "${HIDAPI_LIBRARY_DIR}/hidapi")
    endif()

    if(EXISTS "${HIDAPI_LIBRARY_FILE}")
        target_link_libraries(ajazz_time_correction_tool PRIVATE "${HIDAPI_LIBRARY_FILE}")
        message(STATUS "Linking against HIDAPI library: ${HIDAPI_LIBRARY_FILE}")
    else()
        # Try with lib prefix for non-Windows if the first attempt failed (e.g. if HIDAPI_LIBRARY_FILE was set to just "hidapi" previously)
        if(NOT WIN32 AND NOT EXISTS "${HIDAPI_LIBRARY_FILE}" AND EXISTS "${HIDAPI_LIBRARY_DIR}/libhidapi.so")
             set(HIDAPI_LIBRARY_FILE "${HIDAPI_LIBRARY_DIR}/libhidapi.so")
             target_link_libraries(ajazz_time_correction_tool PRIVATE "${HIDAPI_LIBRARY_FILE}")
             message(STATUS "Linking against HIDAPI library (fallback search): ${HIDAPI_LIBRARY_FILE}")
        elseif(NOT WIN32 AND NOT EXISTS "${HIDAPI_LIBRARY_FILE}" AND EXISTS "${HIDAPI_LIBRARY_DIR}/libhidapi.dylib") # macOS fallback
             set(HIDAPI_LIBRARY_FILE "${HIDAPI_LIBRARY_DIR}/libhidapi.dylib")
             target_link_libraries(ajazz_time_correction_tool PRIVATE "${HIDAPI_LIBRARY_FILE}")
             message(STATUS "Linking against HIDAPI library (fallback search): ${HIDAPI_LIBRARY_FILE}")
        else()
             message(FATAL_ERROR "HIDAPI library not found at specified HIDAPI_LIBRARY_DIR: ${HIDAPI_LIBRARY_FILE} (or common variations). Ensure the correct library file name and path are provided via -DHIDAPI_LIBRARY_DIR.")
        endif()
    endif()
else()
    # Fallback to find_package if variables are not explicitly set or are empty
    message(STATUS "HIDAPI_INCLUDE_DIR or HIDAPI_LIBRARY_DIR not set/empty. Attempting find_package(HIDAPI).")
    # This is for local development convenience. CI should always provide the paths.
    find_package(HIDAPI REQUIRED)
    if(HIDAPI_FOUND) # find_package(REQUIRED) will terminate if not found, but good practice
        message(STATUS "Found HIDAPI via find_package.")
        message(STATUS "  HIDAPI Libraries: ${HIDAPI_LIBRARIES}")
        message(STATUS "  HIDAPI Include Dirs: ${HIDAPI_INCLUDE_DIRS}")
        target_include_directories(ajazz_time_correction_tool PRIVATE ${HIDAPI_INCLUDE_DIRS})
        target_link_libraries(ajazz_time_correction_tool PRIVATE ${HIDAPI_LIBRARIES})
    else()
        # This else block might be technically unreachable due to REQUIRED in find_package,
        # but kept for logical completeness or if REQUIRED is ever removed.
        message(FATAL_ERROR "HIDAPI not found via find_package. Please install HIDAPI development package (e.g., libhidapi-dev) or provide HIDAPI_INCLUDE_DIR and HIDAPI_LIBRARY_DIR CMake variables.")
    endif()
endif()
# --- End HIDAPI Linking ---