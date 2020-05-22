/** @file

  Copyright (c) 2014 - 2016, Intel Corporation. All rights reserved.<BR>
  This program and the accompanying materials
  are licensed and made available under the terms and conditions of the BSD License
  which accompanies this distribution.  The full text of the license may be found at
  http://opensource.org/licenses/bsd-license.php.

  THE PROGRAM IS DISTRIBUTED UNDER THE BSD LICENSE ON AN "AS IS" BASIS,
  WITHOUT WARRANTIES OR REPRESENTATIONS OF ANY KIND, EITHER EXPRESS OR IMPLIED.

**/

#ifndef _FSP_COMMON_LIB_H_
#define _FSP_COMMON_LIB_H_

#include <FspGlobalData.h>
#include <FspMeasurePointId.h>

/**
  This function sets the FSP global data pointer.

  @param[in] FspData       Fsp global data pointer.

**/
VOID
EFIAPI
SetFspGlobalDataPointer (
  IN FSP_GLOBAL_DATA   *FspData
  );

/**
  This function gets the FSP global data pointer.

**/
FSP_GLOBAL_DATA *
EFIAPI
GetFspGlobalDataPointer (
  VOID
  );

/**
  This function gets back the FSP API first parameter passed by the bootlaoder.

  @retval ApiParameter FSP API first parameter passed by the bootlaoder.
**/
UINT32
EFIAPI
GetFspApiParameter (
  VOID
  );

/**
  This function gets back the FSP API second parameter passed by the bootlaoder.

  @retval ApiParameter FSP API second parameter passed by the bootlaoder.
**/
UINT32
EFIAPI
GetFspApiParameter2 (
  VOID
  );

/**
  This function sets the FSP API parameter in the stack.

   @param[in] Value       New parameter value.

**/
VOID
EFIAPI
SetFspApiParameter (
  IN UINT32      Value
  );

/**
  This function set the API status code returned to the BootLoader.

  @param[in] ReturnStatus       Status code to return.

**/
VOID
EFIAPI
SetFspApiReturnStatus (
  IN UINT32  ReturnStatus
  );

/**
  This function sets the context switching stack to a new stack frame.

  @param[in] NewStackTop       New core stack to be set.

**/
VOID
EFIAPI
SetFspCoreStackPointer (
  IN VOID   *NewStackTop
  );

/**
  This function sets the platform specific data pointer.

  @param[in] PlatformData       Fsp platform specific data pointer.

**/
VOID
EFIAPI
SetFspPlatformDataPointer (
  IN VOID   *PlatformData
  );

/**
  This function gets the platform specific data pointer.

   @param[in] PlatformData       Fsp platform specific data pointer.

**/
VOID *
EFIAPI
GetFspPlatformDataPointer (
  VOID
  );

/**
  This function sets the UPD data pointer.

  @param[in] UpdDataPtr   UPD data pointer.
**/
VOID
EFIAPI
SetFspUpdDataPointer (
  IN VOID    *UpdDataPtr
  );

/**
  This function gets the UPD data pointer.

  @return UpdDataPtr   UPD data pointer.
**/
VOID *
EFIAPI
GetFspUpdDataPointer (
  VOID
  );

/**
  This function sets the memory init UPD data pointer.

  @param[in] MemoryInitUpdPtr   memory init UPD data pointer.
**/
VOID
EFIAPI
SetFspMemoryInitUpdDataPointer (
  IN VOID    *MemoryInitUpdPtr
  );

/**
  This function gets the memory init UPD data pointer.

  @return memory init UPD data pointer.
**/
VOID *
EFIAPI
GetFspMemoryInitUpdDataPointer (
  VOID
  );

/**
  This function sets the silicon init UPD data pointer.

  @param[in] SiliconInitUpdPtr   silicon init UPD data pointer.
**/
VOID
EFIAPI
SetFspSiliconInitUpdDataPointer (
  IN VOID    *SiliconInitUpdPtr
  );

/**
  This function gets the silicon init UPD data pointer.

  @return silicon init UPD data pointer.
**/
VOID *
EFIAPI
GetFspSiliconInitUpdDataPointer (
  VOID
  );

/**
  Set FSP measurement point timestamp.

  @param[in] Id       Measurement point ID.

  @return performance timestamp.
**/
UINT64
EFIAPI
SetFspMeasurePoint (
  IN UINT8  Id
  );

/**
  This function gets the FSP info header pointer.

  @retval FspInfoHeader   FSP info header pointer
**/
FSP_INFO_HEADER *
EFIAPI
GetFspInfoHeader (
  VOID
  );

/**
  This function sets the FSP info header pointer.

  @param[in] FspInfoHeader   FSP info header pointer
**/
VOID
EFIAPI
SetFspInfoHeader (
  FSP_INFO_HEADER *FspInfoHeader
  );

/**
  This function gets the FSP info header pointer from the API context.

  @retval FspInfoHeader   FSP info header pointer
**/
FSP_INFO_HEADER *
EFIAPI
GetFspInfoHeaderFromApiContext (
  VOID
  );

/**
  This function gets the CfgRegion data pointer.

  @return CfgRegion data pointer.
**/
VOID *
EFIAPI
GetFspCfgRegionDataPointer (
  VOID
  );

/**
  This function gets FSP API calling mode.

  @retval API calling mode
**/
UINT8
EFIAPI
GetFspApiCallingIndex (
  VOID
  );

/**
  This function sets FSP API calling mode.

  @param[in] Index     API calling index
**/
VOID
EFIAPI
SetFspApiCallingIndex (
  UINT8  Index
  );

/**
  This function gets FSP Phase StatusCode.

  @retval StatusCode
**/
UINT32
EFIAPI
GetPhaseStatusCode (
  VOID
  );


/**
  This function sets FSP Phase StatusCode.

  @param[in] Mode     Phase StatusCode
**/
VOID
EFIAPI
SetPhaseStatusCode (
  UINT32  StatusCode
  );

/**
  This function updates the return status of the FSP API with requested reset type and returns to Boot Loader.

  @param[in] FspResetType     Reset type that needs to returned as API return status

**/
VOID
EFIAPI
FspApiReturnStatusReset (
  IN UINT32   FspResetType
  );
#endif
