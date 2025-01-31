KENDRYTE_BOOT ?= kendryte_boot

cibuild: mainboard
# TODO
nop:
		echo nope...

ciclippy: nop
citest: nop
checkformat: nop

bt0:
	make -C bt0

main:
	make -C main

build: bt0 main

mainboard: build

run_bt0:
	make -C bt0 run

run_main:
	make -C main run

load_payload:
	sleep 2
	$(KENDRYTE_BOOT) load -a 0x200000 $(PAYLOAD)

run: run_bt0 load_payload run_main
