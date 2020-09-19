#!/usr/bin/env bash

echo "Mainboard;Size(bytes)"
for BOOTBLOB in $(find . -name bootblob.bin)
do
  SIZE=$(stat -c"%s" $BOOTBLOB)
  if [[ $BOOTBLOB =~ \./src/mainboard/([^/]+)/([^/]+).* ]]; then 
    echo "${BASH_REMATCH[1]}/${BASH_REMATCH[2]};${SIZE}"
  fi 
done
