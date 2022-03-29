#!/usr/bin/env bash

echo "Mainboard;Size(bytes)"
for BOOTBLOB in $(find ./target -name *-bootblob.bin)
do
  SIZE=$(stat -c"%s" $BOOTBLOB)
  if [[ $BOOTBLOB =~ \./target/.*/([^/]+)-([^/]+)-bootblob.bin ]]; then 
    echo "${BASH_REMATCH[1]}/${BASH_REMATCH[2]};${SIZE}"
  fi 
done
