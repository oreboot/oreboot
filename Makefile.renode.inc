# This Makefile is included by src/mainboards/VENDOR/BOARD/Makefile for renode
# recipes.

renode_run: mainboard
	echo $(IMAGE)
	$(RENODE) $(RENODE_FLAGS)
