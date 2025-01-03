set(CMAKE_SYSTEM_NAME Generic)
set(CMAKE_SYSTEM_PROCESSOR RISC-V)

message("** find toolchain")
find_program(CROSS_CC "${CROSS_PREFIX}gcc")
find_program(CROSS_CXX "${CROSS_PREFIX}g++")
find_program(CROSS_OBJDUMP "${CROSS_PREFIX}objdump")
find_program(CROSS_OBJCOPY "${CROSS_PREFIX}objcopy")
find_program(CROSS_SIZE "${CROSS_PREFIX}size")
if (CROSS_CC AND CROSS_CXX AND CROSS_OBJCOPY AND CROSS_OBJCOPY AND CROSS_SIZE)
    message("@@ found toolchain ${CROSS_CC}")
    set(CMAKE_C_COMPILER ${CROSS_CC})
    set(CMAKE_CXX_COMPILER ${CROSS_CXX})
endif()

set(TARGET_FLAGS "-march=${TARGET_ARCH} -mabi=ilp32 -mcmodel=medany -msmall-data-limit=8 -mno-save-restore")

set(COMMON_FLAGS "-fdata-sections -ffunction-sections")

set(FLAGS_DEBUG "-Og -g3")
set(FLAGS_RELEASE "-O3")
set(FLAGS_SIZE "-Os")

set(CMAKE_C_FLAGS "${TARGET_FLAGS} ${OPT_FLAGS} ${COMMON_FLAGS}")
set(CMAKE_C_FLAGS_DEBUG ${FLAGS_DEBUG})
set(CMAKE_C_FLAGS_RELEASE ${FLAGS_RELEASE})
set(CMAKE_C_FLAGS_MINSIZEREL ${FLAGS_SIZE})
set(CMAKE_CXX_FLAGS "${TARGET_FLAGS} ${OPT_FLAGS} ${COMMON_FLAGS} -fno-rtti -fno-exceptions")
set(CMAKE_CXX_FLAGS_DEBUG ${FLAGS_DEBUG})
set(CMAKE_CXX_FLAGS_RELEASE ${FLAGS_RELEASE})
set(CMAKE_CXX_FLAGS_MINSIZEREL ${FLAGS_SIZE})
SET(CMAKE_ASM_FLAGS "${CFLAGS} ${TARGET_FLAGS} -x assembler-with-cpp")

set(LD_FLAGS "-Wl,--gc-sections -Wl,--print-memory-usage -nostartfiles")

set(CMAKE_EXE_LINKER_FLAGS "${TARGET_FLAGS} --specs=nano.specs --specs=nosys.specs -ffreestanding ${LD_FLAGS}" CACHE INTERNAL "")

set(CMAKE_FIND_ROOT_PATH_MODE_PROGRAM NEVER)
set(CMAKE_FIND_ROOT_PATH_MODE_LIBRARY ONLY)
set(CMAKE_FIND_ROOT_PATH_MODE_INCLUDE ONLY)
set(CMAKE_FIND_ROOT_PATH_MODE_PACKAGE ONLY)

set(default_build_type "Debug")
if(NOT CMAKE_BUILD_TYPE AND NOT CMAKE_CONFIGURATION_TYPES)
  message(STATUS "Setting build type to '${default_build_type}' as none was specified.")
  set(CMAKE_BUILD_TYPE "${default_build_type}" CACHE
      STRING "Choose the type of build." FORCE)
  # Set the possible values of build type for cmake-gui
  set_property(CACHE CMAKE_BUILD_TYPE PROPERTY STRINGS
    "Debug" "Release" "MinSizeRel" "RelWithDebInfo")
endif()
