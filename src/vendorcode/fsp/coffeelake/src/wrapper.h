// These are the header files provided with Coffeelake FSP.
// The order of these includes matters for building.
#include <FspmUpd.h>
#include <FspsUpd.h>
#include <FsptUpd.h>
#include <FspUpd.h>
#include <FirmwareVersionInfoHob.h>
#include <FspInfoHob.h>
// This definition is missing from Coffeelake preventing GpioSampleDef.h from building.
//typedef struct {
//  GPIO_PAD           GpioPad;
//  GPIO_CONFIG        GpioConfig;
//} GPIO_INIT_CONFIG;
//#include <GpioSampleDef.h>
#include <MemInfoHob.h>
// BOOT_MODE
#include <Pi/PiBootMode.h>
