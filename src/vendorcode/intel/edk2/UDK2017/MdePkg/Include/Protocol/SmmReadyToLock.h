/** @file
  SMM Ready To Lock protocol introduced in the PI 1.2 specification.

  According to PI 1.4a specification, this SMM protocol indicates that
  SMM resources and services that should not be used by the third party
  code are about to be locked.
  This protocol is a mandatory protocol published by the SMM Foundation
  code when the system is preparing to lock certain resources and interfaces
  in anticipation of the invocation of 3rd party extensible modules.
  This protocol is an SMM counterpart of the DXE SMM Ready to Lock Protocol.
  This protocol prorogates resource locking notification into SMM environment.
  This protocol is installed after installation of the SMM End of DXE Protocol.

  Copyright (c) 2009 - 2016, Intel Corporation. All rights reserved.<BR>
  This program and the accompanying materials
  are licensed and made available under the terms and conditions of the BSD License
  which accompanies this distribution.  The full text of the license may be found at
  http://opensource.org/licenses/bsd-license.php

  THE PROGRAM IS DISTRIBUTED UNDER THE BSD LICENSE ON AN "AS IS" BASIS,
  WITHOUT WARRANTIES OR REPRESENTATIONS OF ANY KIND, EITHER EXPRESS OR IMPLIED.

**/

#ifndef _SMM_READY_TO_LOCK_H_
#define _SMM_READY_TO_LOCK_H_

#include <Protocol/MmReadyToLock.h>

#define EFI_SMM_READY_TO_LOCK_PROTOCOL_GUID EFI_MM_READY_TO_LOCK_PROTOCOL_GUID

extern EFI_GUID gEfiSmmReadyToLockProtocolGuid;

#endif
