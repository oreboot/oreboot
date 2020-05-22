#ifndef __BDK_ARCH_H__
#define __BDK_ARCH_H__
/***********************license start***********************************
* Copyright (c) 2003-2017  Cavium Inc. (support@cavium.com). All rights
* reserved.
*
*
* Redistribution and use in source and binary forms, with or without
* modification, are permitted provided that the following conditions are
* met:
*
*   * Redistributions of source code must retain the above copyright
*     notice, this list of conditions and the following disclaimer.
*
*   * Redistributions in binary form must reproduce the above
*     copyright notice, this list of conditions and the following
*     disclaimer in the documentation and/or other materials provided
*     with the distribution.
*
*   * Neither the name of Cavium Inc. nor the names of
*     its contributors may be used to endorse or promote products
*     derived from this software without specific prior written
*     permission.
*
* This Software, including technical data, may be subject to U.S. export
* control laws, including the U.S. Export Administration Act and its
* associated regulations, and may be subject to export or import
* regulations in other countries.
*
* TO THE MAXIMUM EXTENT PERMITTED BY LAW, THE SOFTWARE IS PROVIDED "AS IS"
* AND WITH ALL FAULTS AND CAVIUM INC. MAKES NO PROMISES, REPRESENTATIONS OR
* WARRANTIES, EITHER EXPRESS, IMPLIED, STATUTORY, OR OTHERWISE, WITH RESPECT
* TO THE SOFTWARE, INCLUDING ITS CONDITION, ITS CONFORMITY TO ANY
* REPRESENTATION OR DESCRIPTION, OR THE EXISTENCE OF ANY LATENT OR PATENT
* DEFECTS, AND CAVIUM SPECIFICALLY DISCLAIMS ALL IMPLIED (IF ANY) WARRANTIES
* OF TITLE, MERCHANTABILITY, NONINFRINGEMENT, FITNESS FOR A PARTICULAR
* PURPOSE, LACK OF VIRUSES, ACCURACY OR COMPLETENESS, QUIET ENJOYMENT,
* QUIET POSSESSION OR CORRESPONDENCE TO DESCRIPTION. THE ENTIRE  RISK
* ARISING OUT OF USE OR PERFORMANCE OF THE SOFTWARE LIES WITH YOU.
***********************license end**************************************/

/**
 * @file
 *
 * Master include file for architecture support. Use bdk.h
 * instead of including this file directly.
 *
 * <hr>$Revision: 49448 $<hr>
 */

#include <arch/byteorder.h>

#ifndef __BYTE_ORDER
 #if (__LITTLE_ENDIAN)
  #define __BYTE_ORDER __LITTLE_ENDIAN
 #elif defined(__BIG_ENDIAN)
  #define __BYTE_ORDER __BIG_ENDIAN
 #endif
#endif

#ifndef __LITTLE_ENDIAN
 #define __LITTLE_ENDIAN 1234
#endif
#ifndef __BIG_ENDIAN
 #define __BIG_ENDIAN 4321
#endif

#include "bdk-require.h"
#ifndef BDK_BUILD_HOST
#include "bdk-asm.h"
#endif
#include "bdk-model.h"
#include "bdk-numa.h"
#include "bdk-csr.h"
#ifndef BDK_BUILD_HOST
#include "bdk-lmt.h"
#endif
#include "bdk-warn.h"
#ifndef BDK_BUILD_HOST
#include "bdk-fuse.h"
#endif

#endif
