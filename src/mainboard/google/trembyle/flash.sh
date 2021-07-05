#!/bin/sh

# flashrom -p ch341a_spi -l layout.txt -i orefull -w out.bin
sudo flashrom -p raiden_debug_spi:target=AP -l layout.txt -i orefull -w out.bin
