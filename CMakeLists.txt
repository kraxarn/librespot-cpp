if (CMAKE_BUILD_TYPE STREQUAL "Debug")
	set(CARGO_CMD cargo build --verbose)
	set(TARGET_DIR "debug")
else ()
	set(CARGO_CMD cargo build --release --verbose)
	set(TARGET_DIR "release")
endif ()

if (ENABLE_LTO)
	set(RUST_FLAGS "-Clinker-plugin-lto" "-Clinker=clang-11" "-Clink-arg=-fuse-ld=lld-11")
endif ()

set(RUST_PART_LIB "${CMAKE_CURRENT_BINARY_DIR}/${TARGET_DIR}/liblibrespot_cpp.a")
set(RUST_PART_CXX "${CMAKE_CURRENT_BINARY_DIR}/librespot_cpp.cpp")

add_library(librespot-cpp STATIC ${RUST_PART_CXX})
add_custom_command(
		OUTPUT ${RUST_PART_CXX}
		COMMAND CARGO_TARGET_DIR=${CMAKE_CURRENT_BINARY_DIR} RUSTFLAGS="${RUST_FLAGS}" ${CARGO_CMD}
		COMMAND cp ${CMAKE_CURRENT_BINARY_DIR}/cxxbridge/librespot-cpp/src/lib.rs.cc
		${RUST_PART_CXX}
		COMMAND cp ${CMAKE_CURRENT_BINARY_DIR}/cxxbridge/librespot-cpp/src/lib.rs.h
		${CMAKE_CURRENT_BINARY_DIR}/librespot_cpp.h
		WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}
)

target_link_libraries(librespot-cpp pthread dl ${RUST_PART_LIB})
