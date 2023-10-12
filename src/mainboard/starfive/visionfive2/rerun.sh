#!/bin/sh

set -e

#KDIR=/home/dama/firmware/RISC-V/StarFive/VisionFive2/linux-6.4-vf2
KDIR=/home/dama/firmware/RISC-V/StarFive/VisionFive2/linux
PORT=/dev/ttyUSB0
#PORT=/dev/ttyACM0

make -C main
make -C bt0 KERNEL_DIR=$KDIR SERIAL=$PORT runwithpayload
picocom -b 115200 $PORT
