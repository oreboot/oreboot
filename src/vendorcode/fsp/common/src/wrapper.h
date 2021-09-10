/**
 * These are defined in the ProcessorBind.h file for each architecture.
 * The goal of this crate is to work regardless of the architecture
 * of the FSP blob.
 */
typedef unsigned long long  UINT64;
typedef long long           INT64;
typedef unsigned int        UINT32;
typedef int                 INT32;
typedef unsigned short      CHAR16;
typedef unsigned short      UINT16;
typedef short               INT16;
typedef unsigned char       BOOLEAN;
typedef unsigned char       UINT8;
typedef char                CHAR8;
typedef char                INT8;

typedef struct {
  UINT32  Data1;
  UINT16  Data2;
  UINT16  Data3;
  UINT8   Data4[8];
} GUID;

typedef GUID EFI_GUID;

#include <Guid/FspHeaderFile.h>
