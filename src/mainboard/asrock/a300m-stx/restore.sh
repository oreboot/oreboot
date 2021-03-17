#!/bin/sh

flashrom -p ch341a_spi -l layout.txt -i bootblock -w in.bin
