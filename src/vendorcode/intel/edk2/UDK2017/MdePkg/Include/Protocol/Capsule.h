/** @file
  Capsule Architectural Protocol as defined in PI1.0a Specification VOLUME 2 DXE

  The DXE Driver that produces this protocol must be a runtime driver.
  The driver is responsible for initializing the CapsuleUpdate() and
  QueryCapsuleCapabilities() fields of the UEFI Runtime Services Table.
  After the two fields of the UEFI Runtime Services Table have been initialized,
  the driver must install the EFI_CAPSULE_ARCH_PROTOCOL_GUID on a new handle
  with a NULL interface pointer. The installation of this protocol informs
  the DXE Foundation that the Capsule related services are now available and
  that the DXE Foundation must update the 32-bit CRC of the UEFI Runtime Services Table.

Copyright (c) 2006 - 2009, Intel Corporation. All rights reserved.<BR>
This program and the accompanying materials
are licensed and made available under the terms and conditions of the BSD License
which accompanies this distribution.  The full text of the license may be found at
http://opensource.org/licenses/bsd-license.php

THE PROGRAM IS DISTRIBUTED UNDER THE BSD LICENSE ON AN "AS IS" BASIS,
WITHOUT WARRANTIES OR REPRESENTATIONS OF ANY KIND, EITHER EXPRESS OR IMPLIED.

**/

#ifndef __ARCH_PROTOCOL_CAPSULE_ARCH_H__
#define __ARCH_PROTOCOL_CAPSULE_ARCH_H__

//
// Global ID for the Capsule Architectural Protocol
//
#define EFI_CAPSULE_ARCH_PROTOCOL_GUID \
  { 0x5053697e, 0x2cbc, 0x4819, {0x90, 0xd9, 0x05, 0x80, 0xde, 0xee, 0x57, 0x54 }}

extern EFI_GUID gEfiCapsuleArchProtocolGuid;

#endif
