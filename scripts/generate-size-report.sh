#!/usr/bin/env bash

echo "Mainboard;Size(bytes)"
for BIN in $(find ./target/*/release/ -name *.bin)
do
  SIZE=$(stat -c"%s" $BIN)
  if [[ $BIN =~ \./target/.*/([^/]+)-([^/]+).bin ]]; then 
    echo "${BASH_REMATCH[1]}/${BASH_REMATCH[2]};${SIZE}"
  fi 
done
