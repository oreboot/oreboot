# Milk-V Duo

## Boards and SoCs

https://milkv.io/docs/duo/overview

There are 3 + 1 boards, based on related SoCs:

|         Board        |   SoC   |                        URL                        |
| -------------------- | ------- | ------------------------------------------------- |
| Milk-V Duo           | CV1800B | https://milkv.io/docs/duo/getting-started/duo     |
| Milk-V Duo 256M      | SG2002  | https://milkv.io/docs/duo/getting-started/duo256m |
| Milk-V Duo S         | SG2000  | https://milkv.io/docs/duo/getting-started/duos    |
| Sipeed LicheeRV Nano | SG2002  | https://wiki.sipeed.com/licheerv-nano             |

All 3 SoCs have similar cores and peripherals.
Note that the first code on the CV1800B runs at a different base address.

Their mask ROM exposes a loader interface via USB, which allows for loading data
and code to be executed. The following tool lets you load binary code:
<https://github.com/orangecms/sg_boot>
- `sg_boot run bt0.bin` will load the first stage
- `sg_boot run --main main.bin` will load later stages

To interpret detailed CPU information, we print out the raw `CPUID` data.
This tool will allow for printing details:
<https://github.com/platform-system-interface/thead_cpuinfo>

## Run the code

Note:
*So far, only the Duo S / SG2000 is really supported, from DRAM init to S-mode.*

We will now assume that you have `sg_boot` in your `$PATH`.

In addition, clone this repo to have a test binary for running under the SBI:
<https://github.com/orangecms/sbitest>

The `Makefile`s here are prepared to load `bt0` and `main`:
1. in `bt0`, run `make run`
2. in `main`, run `make run`
3. in `sbitest`, run `make bin && sg_boot run --main test.bin`
