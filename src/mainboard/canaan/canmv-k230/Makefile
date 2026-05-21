KENDRYTE_BOOT ?= kendryte_boot

cibuild: mainboard
# TODO
nop:
		echo nope...

ciclippy: nop
citest: nop
checkformat: nop

mainboard:
	$(MAKE) -C bt0
	$(MAKE) -C main

run_bt0:
	$(MAKE) -C bt0 run

run_main:
	$(MAKE) -C main run

load_payload:
	sleep 2
	$(KENDRYTE_BOOT) load -a 0x200000 $(PAYLOAD)

run: run_bt0 load_payload run_main
