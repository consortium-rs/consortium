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
