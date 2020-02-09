help:
	@echo 'Make options:'
	@echo '  # Build a single board'
	@echo '  make VENDOR/BOARD'
	@echo '  # This is equivalent to'
	@echo '  cd src/mainboard/VENDOR/BOARD && make'
	@echo '  # Build all mainboards'
	@echo '  make mainboards'
	@echo '  # Build everything in parallel'
	@echo '  make -j mainboards'
	@echo '  # Build debug mode'
	@echo '  MODE=debug make mainboards'

MAINBOARDS := $(shell cd src/mainboard && find -name Makefile -printf "%h ")

.PHONY: mainboards $(MAINBOARDS)
mainboards: $(MAINBOARDS)

$(MAINBOARDS):
	cd src/mainboard/$@ && make

clean:
	rm -rf $(wildcard src/mainboard/*/*/target)
