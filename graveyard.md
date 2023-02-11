# oreboot Graveyard

We iteratively walk through various approaches to oreboot's design.
Earlier attempts are documented below for reference. The code may still be
useful in general, though requires rework to fit into the current design.

## Removals

The following mainboards have been removed:

- Aspeed ast25x0
- Nuvoton npcm7xx
- OpenTitan crb, [Documentation](Documentation/opentitan/README.md)
- SiFive HiFive Unleashed, [Documentation](Documentation/sifive/setup.md)

## History

```
Path                               Deleting Commit
================================== ========================================
src/lib/uefi                       ee20512302caca7395c08d6145d412ece3879f1f
3rdparty/fsp                       ee20512302caca7395c08d6145d412ece3879f1f
3rdparty/fspsdk                    ee20512302caca7395c08d6145d412ece3879f1f
src/vendorcode/fsp                 d9959f3d5851e1237066f31ece7be0c7f8827413
src/mainboard/emulation/qemu-fsp   0d6302b7b101da1191c3e20352b302d0c684e9b4
src/mainboard/emulation/qemu-fsp32 0d6302b7b101da1191c3e20352b302d0c684e9b4
src/vendorcode/fsp/qemu            0d6302b7b101da1191c3e20352b302d0c684e9b4
src/mainboard/aaeon/upxtreme       95b00cf07894ac3cbdcb70af6c8acc84d26f057d
src/vendorcode/fsp/coffeelake      95b00cf07894ac3cbdcb70af6c8acc84d26f057d
src/vendorcode/fsp/tigerlake       fc291b5c967170edf9414822c3e2195b3e93020d

src/mainboard/aaeon/upsquared	c00613705d5de9235835708a6b2a3ab76da23168
src/mainboard/amd/romecrb	c00613705d5de9235835708a6b2a3ab76da23168
src/mainboard/asrock/a300m-stx	c00613705d5de9235835708a6b2a3ab76da23168
src/mainboard/ast/ast25x0	c00613705d5de9235835708a6b2a3ab76da23168
src/mainboard/google/trembyle	c00613705d5de9235835708a6b2a3ab76da23168
src/mainboard/nuvoton/npcm7xx	c00613705d5de9235835708a6b2a3ab76da23168
src/mainboard/opentitan/crb	c00613705d5de9235835708a6b2a3ab76da23168
src/mainboard/seeed/beaglev	c00613705d5de9235835708a6b2a3ab76da23168
src/mainboard/sifive/hifive	c00613705d5de9235835708a6b2a3ab76da23168
```
