// These are the header files provided with Apollolake FSP.
// The order of these includes matters for building.
#include <FspmUpd.h>
#include <FspsUpd.h>
#include <FsptUpd.h>
#include <FspUpd.h>

typedef unsigned long u32;

#include <GpioSampleDef.h>

// This definition is missing from Apollolake preventing GpioSampleDef.h from building.
/*
#include <FirmwareVersionInfoHob.h>
#include <FspInfoHob.h>
//typedef struct {
//  GPIO_PAD           GpioPad;
//  GPIO_CONFIG        GpioConfig;
//} GPIO_INIT_CONFIG;
#include <MemInfoHob.h>
// BOOT_MODE
#include <Pi/PiBootMode.h>
*/
