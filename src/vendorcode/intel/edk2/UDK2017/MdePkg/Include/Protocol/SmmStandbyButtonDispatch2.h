/** @file
  SMM Standby Button Dispatch2 Protocol as defined in PI 1.1 Specification
  Volume 4 System Management Mode Core Interface.

  This protocol provides the parent dispatch service for the standby button SMI source generator.

  Copyright (c) 2009 - 2010, Intel Corporation. All rights reserved.<BR>
  This program and the accompanying materials
  are licensed and made available under the terms and conditions of the BSD License
  which accompanies this distribution.  The full text of the license may be found at
  http://opensource.org/licenses/bsd-license.php

  THE PROGRAM IS DISTRIBUTED UNDER THE BSD LICENSE ON AN "AS IS" BASIS,
  WITHOUT WARRANTIES OR REPRESENTATIONS OF ANY KIND, EITHER EXPRESS OR IMPLIED.

  @par Revision Reference:
  This protocol is from PI Version 1.1.

**/

#ifndef _SMM_STANDBY_BUTTON_DISPATCH2_H_
#define _SMM_STANDBY_BUTTON_DISPATCH2_H_

#include <Protocol/MmStandbyButtonDispatch.h>

#define EFI_SMM_STANDBY_BUTTON_DISPATCH2_PROTOCOL_GUID EFI_MM_STANDBY_BUTTON_DISPATCH_PROTOCOL_GUID

///
/// The dispatch function's context.
///
typedef EFI_MM_STANDBY_BUTTON_REGISTER_CONTEXT EFI_SMM_STANDBY_BUTTON_REGISTER_CONTEXT;

typedef EFI_MM_STANDBY_BUTTON_DISPATCH_PROTOCOL EFI_SMM_STANDBY_BUTTON_DISPATCH2_PROTOCOL;

typedef EFI_MM_STANDBY_BUTTON_REGISTER EFI_SMM_STANDBY_BUTTON_REGISTER2;

typedef EFI_MM_STANDBY_BUTTON_UNREGISTER EFI_SMM_STANDBY_BUTTON_UNREGISTER2;

extern EFI_GUID gEfiSmmStandbyButtonDispatch2ProtocolGuid;

#endif

