/** @file
  MM Ready To Lock protocol introduced in the PI 1.5 specification.

  This protocol is a mandatory protocol published by the MM Foundation
  code when the system is preparing to lock certain resources and interfaces
  in anticipation of the invocation of 3rd party extensible modules.
  This protocol is an MM counterpart of the DXE MM Ready to Lock Protocol.
  This protocol prorogates resource locking notification into MM environment.
  This protocol is installed after installation of the MM End of DXE Protocol.

  Copyright (c) 2017, Intel Corporation. All rights reserved.<BR>
  This program and the accompanying materials
  are licensed and made available under the terms and conditions of the BSD License
  which accompanies this distribution.  The full text of the license may be found at
  http://opensource.org/licenses/bsd-license.php

  THE PROGRAM IS DISTRIBUTED UNDER THE BSD LICENSE ON AN "AS IS" BASIS,
  WITHOUT WARRANTIES OR REPRESENTATIONS OF ANY KIND, EITHER EXPRESS OR IMPLIED.

**/

#ifndef _MM_READY_TO_LOCK_H_
#define _MM_READY_TO_LOCK_H_

#define EFI_MM_READY_TO_LOCK_PROTOCOL_GUID \
  { \
    0x47b7fa8c, 0xf4bd, 0x4af6, { 0x82, 0x00, 0x33, 0x30, 0x86, 0xf0, 0xd2, 0xc8 } \
  }

extern EFI_GUID gEfiMmReadyToLockProtocolGuid;

#endif
