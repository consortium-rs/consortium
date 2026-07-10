TARGET      = stm32mp257f_cm33
BUILD_DIR   = build

TOOLCHAIN   = arm-none-eabi-
CC          = $(TOOLCHAIN)gcc
AS          = $(TOOLCHAIN)gcc -x assembler-with-cpp
OBJCOPY     = $(TOOLCHAIN)objcopy
SIZE        = $(TOOLCHAIN)size

# --- Sources ---
C_SOURCES = \
$(wildcard CM33/Core/Src/*.c) \
Common/System/system_stm32mp2xx_m33.c \
$(wildcard Drivers/STM32MP2xx_HAL_Driver/Src/*.c) \
Utilities/CoproSync/copro_sync.c

ASM_SOURCES = CM33/Core/Startup/startup_stm32mp257fakx.s

# --- Includes ---
C_INCLUDES = \
-ICM33/Core/Inc \
-IDrivers/CMSIS/Device/ST/STM32MP2xx/Include \
-IDrivers/CMSIS/Include \
-IDrivers/STM32MP2xx_HAL_Driver/Inc \
-IDrivers/STM32MP2xx_HAL_Driver/Inc/Legacy \
-IUtilities/CoproSync

# --- MCU / defines ---
MCU = -mcpu=cortex-m33 -mthumb -mfpu=fpv5-sp-d16 -mfloat-abi=hard
DEFS = -DUSE_HAL_DRIVER -DSTM32MP257Fxx -DCORE_CM33

CFLAGS  = $(MCU) $(DEFS) $(C_INCLUDES) -Wall -O2 -ffunction-sections -fdata-sections -g -std=gnu11
ASFLAGS = $(MCU) -g

LDSCRIPT = CM33/STM32MP257FAKX_RAM.ld
LDFLAGS  = $(MCU) -specs=nano.specs -T$(LDSCRIPT) \
           -Wl,-Map=$(BUILD_DIR)/$(TARGET).map,--cref -Wl,--gc-sections

OBJECTS  = $(addprefix $(BUILD_DIR)/,$(notdir $(C_SOURCES:.c=.o)))
OBJECTS += $(addprefix $(BUILD_DIR)/,$(notdir $(ASM_SOURCES:.s=.o)))
vpath %.c $(sort $(dir $(C_SOURCES)))
vpath %.s $(sort $(dir $(ASM_SOURCES)))

all: $(BUILD_DIR)/$(TARGET).elf

$(BUILD_DIR)/%.o: %.c Makefile | $(BUILD_DIR)
	$(CC) -c $(CFLAGS) $< -o $@

$(BUILD_DIR)/%.o: %.s Makefile | $(BUILD_DIR)
	$(AS) -c $(ASFLAGS) $< -o $@

$(BUILD_DIR)/$(TARGET).elf: $(OBJECTS) $(LDSCRIPT)
	$(CC) $(OBJECTS) $(LDFLAGS) -o $@
	$(SIZE) $@

$(BUILD_DIR):
	mkdir -p $@

clean:
	rm -rf $(BUILD_DIR)

.PHONY: all clean