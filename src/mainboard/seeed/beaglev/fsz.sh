#!/bin/bash

function handle_file {
	inFile=$1
	echo inFile: $inFile
	outFile=$inFile.out

	inSize=`stat -c "%s" $inFile`
	inSize32HexBe=`printf "%08x\n" $inSize`
	inSize32HexLe=${inSize32HexBe:6:2}${inSize32HexBe:4:2}${inSize32HexBe:2:2}${inSize32HexBe:0:2}
	echo "inSize: $inSize (0x$inSize32HexBe, LE:0x$inSize32HexLe)"

	echo $inSize32HexLe | xxd -r -ps > $outFile
	cat $inFile >> $outFile
	echo outFile: $outFile

	outSize=`stat -c "%s" $outFile`
	outSize32HexBe=`printf "%08x\n" $outSize`
	echo "outSize: $outSize (0x$outSize32HexBe)"
}

if [ "$1" = "" -o "$1" = "--help" ]; then
	echo "Add file size(32bits, Little Endian) before the content."
	echo "Usage: ./fsz.sh <file>"
	exit 1
fi

handle_file "$@"
