/** @file
  This file defines the SPI SMM Host Controller Protocol.

  Copyright (c) 2017, Intel Corporation. All rights reserved.<BR>
  This program and the accompanying materials
  are licensed and made available under the terms and conditions of the BSD
  License which accompanies this distribution. The full text of the license may
  be found at http://opensource.org/licenses/bsd-license.php

  THE PROGRAM IS DISTRIBUTED UNDER THE BSD LICENSE ON AN "AS IS" BASIS,
  WITHOUT WARRANTIES OR REPRESENTATIONS OF ANY KIND, EITHER EXPRESS OR IMPLIED.

  @par Revision Reference:
    This Protocol was introduced in UEFI PI Specification 1.6.

**/

#ifndef __SPI_SMM_HC_H__
#define __SPI_SMM_HC_H__

#include <Protocol/SpiHc.h>

///
/// Global ID for the SPI SMM Host Controller Protocol
///
#define EFI_SPI_SMM_HC_PROTOCOL_GUID  \
  { 0xe9f02217, 0x2093, 0x4470,       \
    { 0x8a, 0x54, 0x5c, 0x2c, 0xff, 0xe7, 0x3e, 0xcb }}

typedef
struct _EFI_SPI_HC_PROTOCOL
EFI_SPI_SMM_HC_PROTOCOL;

extern EFI_GUID gEfiSpiSmmHcProtocolGuid;

#endif // __SPI_SMM_HC_H__
