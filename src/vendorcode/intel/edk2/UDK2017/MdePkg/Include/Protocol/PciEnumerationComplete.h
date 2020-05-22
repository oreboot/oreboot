/** @file
  PCI Enumeration Complete Protocol as defined in the PI 1.1 specification.
  This protocol indicates that pci enumeration complete

  Copyright (c) 2009, Intel Corporation. All rights reserved.<BR>
  This program and the accompanying materials
  are licensed and made available under the terms and conditions of the BSD License
  which accompanies this distribution.  The full text of the license may be found at
  http://opensource.org/licenses/bsd-license.php

  THE PROGRAM IS DISTRIBUTED UNDER THE BSD LICENSE ON AN "AS IS" BASIS,
  WITHOUT WARRANTIES OR REPRESENTATIONS OF ANY KIND, EITHER EXPRESS OR IMPLIED.

  @par Revision Reference:
  This Protocol is defined in UEFI Platform Initialization Specification 1.2
  Volume 5: Standards

**/

#ifndef _PCI_ENUMERATION_COMPLETE_H_
#define _PCI_ENUMERATION_COMPLETE_H_

#define EFI_PCI_ENUMERATION_COMPLETE_GUID \
  {  \
   0x30cfe3e7, 0x3de1, 0x4586, { 0xbe, 0x20, 0xde, 0xab, 0xa1, 0xb3, 0xb7, 0x93  } \
  }

extern EFI_GUID gEfiPciEnumerationCompleteProtocolGuid;

#endif
