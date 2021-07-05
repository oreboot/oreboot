#!/bin/sh
sudo flashrom -p raiden_debug_spi:target=AP -l layout.txt -i bootblock -w in.bin

