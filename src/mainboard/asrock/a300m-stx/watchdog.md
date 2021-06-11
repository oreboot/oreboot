## Registers

p 486

9.2.10.2 Power Management (PM) Registers

Table 81: ACPI MMIO Space Allocation

```
0x0BFF-0x0B00
```

The PM register space is accessed through two methods:
 • Indirect IO access through IOCD6 [PM_Index] and IOCD7 [PM_Data]. Software
   first programs the offset into the index register IOCD6 and then reads from
   or writes to the data register IOCD7.
 • Direct memory mapped access through the AcpiMmio region. The PM registers
   range from FED8_0000h+300h to FED8_0000h+3FFh. See PMx04[MmioEn] for details
   on the AcpiMmio region.

PMx000 [DecodeEn] (FCH::PM::PmDecodeEn)
Read-write. Reset: 2302_0B10h.
`_aliasHOST; PMx000; PM=FED8_0300h`

27:26 WatchDogOptions. Read-write. Reset: 0h. Enable for normal WatchDog Timer operation.
  ValidValues:
  Value Description
  0h Enable WatchDog Timer.
  3h-1h Reserved.

7 WatchdogTmrEn. Read-write. Reset: 0. 1=Enable IOAPIC memory (FEB0_0000h ~ FEB0_0003h) decoding, and enable WatchDog Timer operation.


PMx048 [PGPwrEnDly] (FCH::PM::WatchdogTimerEn)
Read-write. Reset: 5935_2B21h.
`_aliasHOST; PMx048; PM=FED8_0300h`

 Bits Description
 31:24 PG1PwrDownDlyTmr. Read-write. Reset: 59h. PG1 Power Down delay timer.
 23:16 XhcPwrEnDlyTmr. Read-write. Reset: 35h. XHC Power Enable delay timer.
 15:8 PG2PwrEnDlyTmr. Read-write. Reset: 2Bh. PG2 Power Enable delay timer.
 8:0 PG1aPwrEnDlyTmr. Read-write. Reset: 21h. PG1a Power En delay timer.

PMx0C0 [S5/Reset Status] (FCH::PM::S5ResetStat)
 Reset: 0000_0800h.

This register shows the source of previous reset.
`_aliasHOST; PMx0C0; PM=FED8_0300h`

25 WatchdogIssueReset. Read,Write-1-to-clear. Reset: 0. Bits[27:16] will be
   cleared by the last reset event except the associated bit will be set.


MSRC001_0074 [CPU Watchdog Timer] (Core::X86::Msr::CpuWdtCfg)

 Read-write. Reset: 0000_0000_0000_0000h.
`_lthree0_core[3:0]; MSRC001_0074`

 Bits Description
 63:7 Reserved.
 6:3 CpuWdtCountSel: CPU watchdog timer count select. Read-write. Reset: 0h.
     CpuWdtCountSel and CpuWdtTimeBase together specify the time period required
     for the WDT to expire. The time period is ((the multiplier specified by
     CpuWdtCountSel) * (the time base specified by CpuWdtTimeBase)). The actual
     timeout period may be anywhere from zero to one increment less than the
     values specified, due to non-deterministic behavior.

     ValidValues:
     Value Description
     0h 4095
     1h 2047
     2h 1023
     3h 511
     4h 255
     5h 127
     6h 63
     7h 31
     8h 8191
     9h 16383
     Fh-Ah Reserved
 2:1 CpuWdtTimeBase: CPU watchdog timer time base. Read-write. Reset: 0h.
     Specifies the time base for the timeout period specified in CpuWdtCountSel.
     ValidValues:
     Value Description
     0h 1.31ms
     1h 1.28us
     3h-2h Reserved
 0 CpuWdtEn: CPU watchdog timer enable. Read-write.
   Reset: 0. Init: BIOS,1. 1=The WDT is enabled.


p 354

9.1.13 Reset Overview

 Below are definitions of the various reset types:
 • Type 0 reset (S5 Reset): RsmRst and UserRst.
 • Type 1 reset (reset initiated by software or system): CF9, KBRst, Sync_flood,
   ASF_remote_reset, Fail_boot, Watchdog Timer reset, toggling of PwrGood
   (SLP_S3#/SLP_S5# remain de-asserted at high), SHUTDOWN command, INIT/PORT92.
 • Type 2 reset (Sleep Reset): S3/S4/S5 reset.
 • Type 3 reset (Fatal_error_reset or reset caused by hardware exception):
   4s-shutdown, thermal trip, ASF_remotePowerDown.
 • Type 4 reset (any reset from above): Type 0 or Type 1 or Type 2 or Type 3.


p 498

PMx0C0 [S5/Reset Status] (FCH::PM::S5ResetStat)

 Reset: 0000_0800h.

This register shows the source of previous reset.
` _aliasHOST; PMx0C0; PM=FED8_0300h`

 Bits Description
 31:28 Reserved.
 27 SyncFlood. Read,Write-1-to-clear. Reset: 0. System reset was caused by a
    SYNC_FLOOD event which was due to an UE error or caused by a SHUTDOWN
    command from CPU if FCH::PM::PciCtl[ShutDownOption]. Bits[27:16] will be
    cleared by the last reset event, except the associated bit will be set.
 26 RemoteResetFromASF. Read,Write-1-to-clear. Reset: 0. System reset was caused
    by a remote RESET command from ASF. Bits[27:16] will be cleared by the last
    reset event, except the associated bit will be set.
 25 WatchdogIssueReset. Read,Write-1-to-clear. Reset: 0. Bits[27:16] will be
    cleared by the last reset event except the associated bit will be set.
