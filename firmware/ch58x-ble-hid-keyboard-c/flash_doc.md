# `flash.c` - Flash Memory Example Overview

This file (`flash.c`) from the CH58x EVT (Evaluation Kit) serves as a demonstration for interacting with the internal
flash memory of the CH58x microcontroller. It showcases fundamental operations for both Data-Flash (often used for
persistent user data, similar to EEPROM) and Flash-ROM (typically for storing firmware code).

## File Structure

*   **`TestBuf`**: A global `uint8_t` array of 1024 bytes used as a temporary buffer for flash read and write
operations.
*   **`DebugInit(void)`**: Configures GPIO pins for UART communication and initializes `UART1_DefInit()`, enabling
debug output via the `PRINT` macro.
*   **`main()`**: The entry point of the application, orchestrating the various flash operations.

## Key Operations Demonstrated

The `main` function executes several sections, primarily focused on flash access:

1.  **System Clock and Debug Initialization**:
    *   Sets the system clock to 60MHz using the PLL (`SetSysClock(CLK_SOURCE_PLL_60MHz)`).
    *   Initializes the debug UART via `DebugInit()`.
    *   Prints the `R8_CHIP_ID`.

2.  **Data-Flash (EEPROM) Operations**:
    *   **Read**: Reads 500 bytes from address 0 of Data-Flash into `TestBuf` (`EEPROM_READ`).
    *   **Erase**: Erases a block of Data-Flash starting at address 0 (`EEPROM_ERASE`), using `EEPROM_BLOCK_SIZE` for
block granularity.
    *   **Write**: Fills `TestBuf` with sequential data and writes 500 bytes to Data-Flash at address 0
(`EEPROM_WRITE`).
    *   Each step includes printing the data to the console for verification.

3.  **Flash-ROM Operations**:
    *   **Unique ID and MAC Address**: Retrieves and prints the unique chip ID (`GET_UNIQUE_ID`) and the device's MAC
address (`GetMACAddress`). These are typically stored in a read-only flash region.
    *   **Read**: Reads 128 bytes from Flash-ROM at an offset of 20KB (`FLASH_ROM_READ`).
    *   **Erase**: Erases a 4KB sector of Flash-ROM starting at the 20KB offset (`FLASH_ROM_ERASE`).
    *   **Write**: Fills `TestBuf` with new sequential data and writes 128 bytes to Flash-ROM at the 20KB offset
(`FLASH_ROM_WRITE`).
    *   **Verify**: Verifies the content written to Flash-ROM (`FLASH_ROM_VERIFY`).
    *   Results of read/write/erase operations are printed to the debug UART.

## Noteworthy Features (Commented Out Sections)

The example also includes commented-out code blocks that highlight additional flash-related functionalities:

*   **User Option Byte Configuration**: Demonstrates how to configure user option bytes, which can control features
like reset enable, boot pin behavior, UART settings, and write protection size (`UserOptionByteConfig`,
`UserOptionByte_Active`).
*   **Close SWD Interface**: Shows how to disable the Serial Wire Debug (SWD) interface (`UserOptionByteClose_SWD`,
`UserOptionByte_Active`), a step often taken in production to secure the device from external debugging access.

## Dependencies

The file relies heavily on the WCH CH58x Standard Peripheral Library (SPL), evidenced by includes like
`CH58x_common.h` and direct calls to functions like `EEPROM_READ`, `FLASH_ROM_WRITE`, `SetSysClock`, etc.

## Usage

This `flash.c` example serves as a reference for developers to implement their own flash memory read, write, and erase
routines, as well as for understanding how to access unique device identifiers and configure hardware options through
flash operations on the CH58x series microcontrollers.

