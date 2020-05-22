/** @file
  Provides services for SMM IO Operation.

  The SMM IO Library provides function for checking if IO resource is accessible inside of SMM.

  Copyright (c) 2017, Intel Corporation. All rights reserved.<BR>
  This program and the accompanying materials
  are licensed and made available under the terms and conditions of the BSD License
  which accompanies this distribution.  The full text of the license may be found at
  http://opensource.org/licenses/bsd-license.php

  THE PROGRAM IS DISTRIBUTED UNDER THE BSD LICENSE ON AN "AS IS" BASIS,
  WITHOUT WARRANTIES OR REPRESENTATIONS OF ANY KIND, EITHER EXPRESS OR IMPLIED.

**/

#ifndef _SMM_IO_LIB_H_
#define _SMM_IO_LIB_H_

/**
  This function check if the MMIO resource is valid per processor architecture and
  valid per platform design.

  @param BaseAddress  The MMIO start address to be checked.
  @param Length       The MMIO length to be checked.
  @param Owner        A GUID representing the owner of the resource.
                      This GUID may be used by producer to correlate the device ownership of the resource.
                      NULL means no specific owner.

  @retval TRUE  This MMIO resource is valid per processor architecture and valid per platform design.
  @retval FALSE This MMIO resource is not valid per processor architecture or valid per platform design.
**/
BOOLEAN
EFIAPI
SmmIsMmioValid (
  IN EFI_PHYSICAL_ADDRESS  BaseAddress,
  IN UINT64                Length,
  IN EFI_GUID              *Owner  OPTIONAL
  );

#endif

