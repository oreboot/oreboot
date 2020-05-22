/** @file
  Present the dependency expression values in PI.

  Copyright (c) 2006 - 2008, Intel Corporation. All rights reserved.<BR>
  This program and the accompanying materials
  are licensed and made available under the terms and conditions of the BSD License
  which accompanies this distribution.  The full text of the license may be found at
  http://opensource.org/licenses/bsd-license.php

  THE PROGRAM IS DISTRIBUTED UNDER THE BSD LICENSE ON AN "AS IS" BASIS,
  WITHOUT WARRANTIES OR REPRESENTATIONS OF ANY KIND, EITHER EXPRESS OR IMPLIED.

  @par Revision Reference:
  PI Version 1.0

**/
#ifndef __PI_DEPENDENCY_H__
#define __PI_DEPENDENCY_H__

///
/// If present, this must be the first and only opcode,
/// EFI_DEP_BEFORE may be used by DXE and SMM drivers.
///
#define EFI_DEP_BEFORE        0x00

///
/// If present, this must be the first and only opcode,
/// EFI_DEP_AFTER may be used by DXE and SMM drivers.
///
#define EFI_DEP_AFTER         0x01

#define EFI_DEP_PUSH          0x02
#define EFI_DEP_AND           0x03
#define EFI_DEP_OR            0x04
#define EFI_DEP_NOT           0x05
#define EFI_DEP_TRUE          0x06
#define EFI_DEP_FALSE         0x07
#define EFI_DEP_END           0x08


///
/// If present, this must be the first opcode,
/// EFI_DEP_SOR is only used by DXE driver.
///
#define EFI_DEP_SOR           0x09

#endif
