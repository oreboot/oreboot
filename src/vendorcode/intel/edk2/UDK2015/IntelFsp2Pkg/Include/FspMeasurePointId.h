/** @file

  Copyright (c) 2014 - 2016, Intel Corporation. All rights reserved.<BR>
  This file and the accompanying materials are licensed and made available under
  the terms and conditions of the BSD License.
  The full text of the license may be found at
  http://opensource.org/licenses/bsd-license.php.

  THIS FILE IS DISTRIBUTED UNDER THE BSD LICENSE ON AN "AS IS" BASIS,
  WITHOUT WARRANTIES OR REPRESENTATIONS OF ANY KIND, EITHER EXPRESS OR IMPLIED.

**/

#ifndef _FSP_MEASURE_POINT_ID_H_
#define _FSP_MEASURE_POINT_ID_H_

//
// 0xD0 - 0xEF are reserved for FSP common measure point
//
#define  FSP_PERF_ID_MRC_INIT_ENTRY               0xD0
#define  FSP_PERF_ID_MRC_INIT_EXIT                (FSP_PERF_ID_MRC_INIT_ENTRY +  1)

#define  FSP_PERF_ID_SYSTEM_AGENT_INIT_ENTRY      0xD8
#define  FSP_PERF_ID_SYSTEM_AGENT_INIT_EXIT       (FSP_PERF_ID_SYSTEM_AGENT_INIT_ENTRY +  1)

#define  FSP_PERF_ID_PCH_INIT_ENTRY               0xDA
#define  FSP_PERF_ID_PCH_INIT_EXIT                (FSP_PERF_ID_PCH_INIT_ENTRY +  1)

#define  FSP_PERF_ID_CPU_INIT_ENTRY               0xE0
#define  FSP_PERF_ID_CPU_INIT_EXIT                (FSP_PERF_ID_CPU_INIT_ENTRY +  1)

#define  FSP_PERF_ID_GFX_INIT_ENTRY               0xE8
#define  FSP_PERF_ID_GFX_INIT_EXIT                (FSP_PERF_ID_GFX_INIT_ENTRY +  1)

#define  FSP_PERF_ID_ME_INIT_ENTRY                0xEA
#define  FSP_PERF_ID_ME_INIT_EXIT                 (FSP_PERF_ID_ME_INIT_ENTRY +  1)

//
// 0xF0 - 0xFF are reserved for FSP API
//
#define  FSP_PERF_ID_API_TEMP_RAM_INIT_ENTRY           0xF0
#define  FSP_PERF_ID_API_TEMP_RAM_INIT_EXIT            (FSP_PERF_ID_API_TEMP_RAM_INIT_ENTRY + 1)

#define  FSP_PERF_ID_API_FSP_MEMORY_INIT_ENTRY         0xF2
#define  FSP_PERF_ID_API_FSP_MEMORY_INIT_EXIT          (FSP_PERF_ID_API_FSP_MEMORY_INIT_ENTRY + 1)

#define  FSP_PERF_ID_API_TEMP_RAM_EXIT_ENTRY           0xF4
#define  FSP_PERF_ID_API_TEMP_RAM_EXIT_EXIT            (FSP_PERF_ID_API_TEMP_RAM_EXIT_ENTRY + 1)

#define  FSP_PERF_ID_API_FSP_SILICON_INIT_ENTRY        0xF6
#define  FSP_PERF_ID_API_FSP_SILICON_INIT_EXIT         (FSP_PERF_ID_API_FSP_SILICON_INIT_ENTRY + 1)

#define  FSP_PERF_ID_API_NOTIFY_POST_PCI_ENTRY         0xF8
#define  FSP_PERF_ID_API_NOTIFY_POST_PCI_EXIT          (FSP_PERF_ID_API_NOTIFY_POST_PCI_ENTRY + 1)

#define  FSP_PERF_ID_API_NOTIFY_READY_TO_BOOT_ENTRY    0xFA
#define  FSP_PERF_ID_API_NOTIFY_READY_TO_BOOT_EXIT     (FSP_PERF_ID_API_NOTIFY_READY_TO_BOOT_ENTRY + 1)

#define  FSP_PERF_ID_API_NOTIFY_END_OF_FIRMWARE_ENTRY  0xFC
#define  FSP_PERF_ID_API_NOTIFY_END_OF_FIRMWARE_EXIT   (FSP_PERF_ID_API_NOTIFY_END_OF_FIRMWARE_ENTRY + 1)

#endif
