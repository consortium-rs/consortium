rm -rf ./data

mkdir -p ./data

mkdir -p ./data/imx95_cm7
mkdir -p ./data/imx93_cm33
mkdir -p ./data/imx95_cm33

cp ./submodules/mcux-devices-imx/i.MX95/**/*cm7*.h ./data/imx95_cm7/
cp ./submodules/mcux-devices-imx/i.MX93/**/*cm33*.h ./data/imx93_cm33/
cp ./submodules/imx-sm/devices/MIMX95/*MIMX95*.h ./data/imx95_cm33/
cp ./submodules/mcux-devices-imx/i.MX95/periph/*.h ./data/imx95_cm7/
cp ./submodules/mcux-devices-imx/i.MX93/periph/*.h ./data/imx93_cm33/

mkdir -p ./data/stm32mp25_cm33
mkdir -p ./data/stm32mp25_cm0p
mkdir -p ./data/stm32mp23_cm33
mkdir -p ./data/stm32mp21_cm33

cp ./submodules/STM32CubeMP2/Drivers/CMSIS/Device/ST/STM32MP2xx/Include/stm32mp21*_cm33.h ./data/stm32mp21_cm33/
cp ./submodules/STM32CubeMP2/Drivers/CMSIS/Device/ST/STM32MP2xx/Include/stm32mp23*_cm33.h ./data/stm32mp23_cm33/
cp ./submodules/STM32CubeMP2/Drivers/CMSIS/Device/ST/STM32MP2xx/Include/stm32mp25*_cm33.h ./data/stm32mp25_cm33/
cp ./submodules/STM32CubeMP2/Drivers/CMSIS/Device/ST/STM32MP2xx/Include/stm32mp25*_cm0.h ./data/stm32mp25_cm0p/
cp ./submodules/STM32CubeMP2/Drivers/CMSIS/Device/ST/STM32MP2xx/Include/stm32mp2xx.h ./data/stm32mp25_cm33/
cp ./submodules/STM32CubeMP2/Drivers/CMSIS/Device/ST/STM32MP2xx/Include/stm32mp2xx.h ./data/stm32mp25_cm0p/
cp ./submodules/STM32CubeMP2/Drivers/CMSIS/Device/ST/STM32MP2xx/Include/stm32mp2xx.h ./data/stm32mp23_cm33/
cp ./submodules/STM32CubeMP2/Drivers/CMSIS/Device/ST/STM32MP2xx/Include/stm32mp2xx.h ./data/stm32mp21_cm33/
cp ./submodules/STM32CubeMP2/Drivers/CMSIS/Device/ST/STM32MP2xx/Include/system_stm32mp2xx.h ./data/stm32mp25_cm33/
cp ./submodules/STM32CubeMP2/Drivers/CMSIS/Device/ST/STM32MP2xx/Include/system_stm32mp2xx.h ./data/stm32mp25_cm0p/
cp ./submodules/STM32CubeMP2/Drivers/CMSIS/Device/ST/STM32MP2xx/Include/system_stm32mp2xx.h ./data/stm32mp23_cm33/
cp ./submodules/STM32CubeMP2/Drivers/CMSIS/Device/ST/STM32MP2xx/Include/system_stm32mp2xx.h ./data/stm32mp21_cm33/

# Pin-mux and DMA-request sources for the signal parsers
# (`parse_signals_stm32.py`, `parse_signals_nxp.py`), staged per die family.
OPD=./submodules/STM32_open_pin_data/mcu
CUBE_HAL=./submodules/STM32CubeMP2/Drivers/STM32MP2xx_HAL_Driver/Inc
MCUX=./submodules/mcux-devices-imx

for family in stm32mp21 stm32mp23 stm32mp25; do
    upper=$(echo "$family" | tr '[:lower:]' '[:upper:]')
    mkdir -p "./data/signal/$family"
    cp "$OPD"/"$upper"*.xml "./data/signal/$family/"
    cp "$OPD"/IP/GPIO-STM32MP2*_gpio_*_Modes.xml "./data/signal/$family/"
    cp "$CUBE_HAL/stm32mp2xx_hal_dma.h" "./data/signal/$family/"
done

mkdir -p ./data/signal/mimx93 ./data/signal/mimx95
cp "$MCUX"/i.MX93/MIMX9352/drivers/fsl_iomuxc.h ./data/signal/mimx93/
cp "$MCUX"/i.MX93/periph/PERI_DMA.h "$MCUX"/i.MX93/periph/PERI_DMA4.h ./data/signal/mimx93/
cp "$MCUX"/i.MX95/MIMX9596/drivers/fsl_iomuxc.h ./data/signal/mimx95/
cp "$MCUX"/i.MX95/MIMX9596/drivers/fsl_edma_soc.h ./data/signal/mimx95/
