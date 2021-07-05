#!/bin/sh

make

as -o start.o start.S
objcopy -O binary start.o start.bin

# insert our code into the original image
go run patch.go
