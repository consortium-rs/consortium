Ethan Wu
successfully landed rust code with defmt and embassy on m33 core of stm32mp257 given little public community support and no swd. nxp imx 95 will also be supported. will apache-2.0 when full hil done.
informing those in april thinking the repo being ai slop solely because of an em dash and thought hardware must explode.
Expand
glitchy and 2 others left and rejoined, Zexiat Kro joined
ocsel
i looove em-dashes and won't get bullied into abandonning them either! 😅
Expand
sroemer joined and left, bandini joined, glitchy and one other left and rejoined
9names
Ethan Wu there is SVDs for stm32mp25: https://www.st.com/en/microcontrollers-microprocessors/stm32mp2-series.html#cad-resources
oh, swd? oops, guess i'm bad at parsing today 🤦
Ethan Wu
yeah the on-board st-link could hardly take the role of debugging. btw nxp imx95 literally didn't have svd available according to my search :-(
so given the combination i took embassy's chiptool as an example for better peripheral reuse
9names
i don't follow. the chip does have swd, is the cm33 not accessable through it?
Ethan Wu
well, it does has a vcp, but st-link couldn't be used for programming, as per the user manual
9names
i have no idea about imx95. but i do have some familiarity with stm32mp1, and it does have SWD and you can use it to debug the mcu core
Ethan Wu
so all we could do is either let the cm33 write to the shared memory or turn off led (ph6) as far as i know, and eventually it worked. another option is to implement a driver for uart5 via pac
'to debug the on-board stm32, no specific hardware configuration is required. programming through stlink-v3ec is not supported with microprocessor products.' perhaps it really can, but on mac the vcp is also fragile, only turning to windows when necessary
btw right now ca35 can kick cm33 interrupt to wake it up. investigating uio to let cm33 successfully kick ca35 up for ipc. ca35 can also forward cm33 defmt message and melt them up together
9names
programming through stlink-v3ec is not supported with microprocessor products

okay, but that's for writing to sdcard/emmc/nor/nand?
if you're writing firmware for the CM33 you'd be writing your program into SRAM anyway.

Ethan Wu
that's simple, though. just scp elf to the ca35 and remoteproc live switch
the only problem is just praying it goes the correct address. whether it enters 0x0 (invalid) or 0x80100000 (valid), dmesg just tells you 'successful' but gpio never flips
9names
do you get any better feedback if you enable the debugfs tracing logs for remoteproc?
you should also be able to attach to the CM33 via SWD and see the processor state directly
Ethan Wu
yeah. it tells me it loads 0x0 so i started to realize the linker script being wrong. then the led flipped.
9names
you should also be able to attach to the CM33 via SWD and see the processor state directly
that's would be a good thing if i knew it yesterday... right now this worked:

1
2
3
4
2026-07-11T13:18:17.601238Z TRACE consortium_dbg::host: HostRing::from_raw_parts(base = 0xffffb3646000, len = 0x2000)
2026-07-11T13:18:17.601463Z TRACE consortium_dbg::host: HostRing::from_raw_parts: descriptor capacity = 0x2000, read_ptr = 0x10, write_ptr = 0x552
2026-07-11T13:18:17.601764Z TRACE consortium::dbg: consortium controller init: time driver bring-up core="cm33"
2026-07-11T13:18:17.601837Z TRACE consortium::dbg: consortium controller init: RCC register block at 0x44200000 core="cm33"
9names
ST posted a series on YouTube on programming the CM33 of stm32mp2 a few days ago. I quickly skimmed it, there's a section on debugging via stm32cubeide.
Might be worth your time to watch it
Ethan Wu
cool. i'll watch it once needed. i currently think that forwarding defmt to a35 can be more convenient than swd :-)
