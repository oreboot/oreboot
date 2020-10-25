#!/bin/sh

as -o serial.o serial.S
objcopy -O binary serial.o serial.bin
# insert our code into the original image
./patch serial.bin@ffe928
