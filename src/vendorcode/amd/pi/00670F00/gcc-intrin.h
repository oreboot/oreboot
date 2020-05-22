/*
 * Copyright (c) 2011, Advanced Micro Devices, Inc. All rights reserved.
 * Copyright (c) 2014, Edward O'Callaghan <eocallaghan@alterapraxis.com>
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are met:
 *     * Redistributions of source code must retain the above copyright
 *       notice, this list of conditions and the following disclaimer.
 *     * Redistributions in binary form must reproduce the above copyright
 *       notice, this list of conditions and the following disclaimer in the
 *       documentation and/or other materials provided with the distribution.
 *     * Neither the name of Advanced Micro Devices, Inc. nor the names of
 *       its contributors may be used to endorse or promote products derived
 *       from this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS" AND
 * ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED
 * WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
 * DISCLAIMED. IN NO EVENT SHALL ADVANCED MICRO DEVICES, INC. BE LIABLE FOR ANY
 * DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES
 * (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES;
 * LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND
 * ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
 * SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 *
 */

#include <check_for_wrapper.h>

#if defined (__GNUC__)
#include <stdint.h>
/* I/O intrin functions.  */
static __inline__ __attribute__((always_inline)) uint8_t __inbyte(uint16_t Port)
{
  uint8_t value;

  __asm__ __volatile__ (
    "in  %1, %0"
    : "=a" (value)
    : "Nd" (Port)
    );

  return value;
}

static __inline__ __attribute__((always_inline)) uint16_t __inword(uint16_t Port)
{
  uint16_t value;

  __asm__ __volatile__ (
    "in  %1, %0"
    : "=a" (value)
    : "Nd" (Port)
    );

  return value;
}

static __inline__ __attribute__((always_inline)) uint32_t __indword(uint16_t Port)
{
  uint32_t value;

  __asm__ __volatile__ (
    "in  %1, %0"
    : "=a" (value)
    : "Nd" (Port)
    );
  return value;

}

static __inline__ __attribute__((always_inline)) void __outbyte(uint16_t Port,uint8_t Data)
{
  __asm__ __volatile__ (
    "out  %0, %1"
    :
    : "a" (Data), "Nd" (Port)
    );
}

static __inline__ __attribute__((always_inline)) void __outword(uint16_t Port,uint16_t Data)
{
  __asm__ __volatile__ (
    "out  %0, %1"
    :
    : "a" (Data), "Nd" (Port)
    );
}

static __inline__ __attribute__((always_inline)) void __outdword(uint16_t Port,uint32_t Data)
{
  __asm__ __volatile__ (
    "out  %0, %1"
    :
    : "a" (Data), "Nd" (Port)
    );
}

static __inline__ __attribute__((always_inline)) void __inbytestring(uint16_t Port,uint8_t *Buffer,unsigned long Count)
{
  __asm__ __volatile__ (
    "rep ; insb"
    : "+D" (Buffer), "+c" (Count)
    : "d"(Port)
    );
}

static __inline__ __attribute__((always_inline)) void __inwordstring(uint16_t Port,uint16_t *Buffer,unsigned long Count)
{
  __asm__ __volatile__ (
    "rep ; insw"
    : "+D" (Buffer), "+c" (Count)
    : "d"(Port)
    );
}

static __inline__ __attribute__((always_inline)) void __indwordstring(uint16_t Port,unsigned long *Buffer,unsigned long Count)
{
  __asm__ __volatile__ (
    "rep ; insl"
    : "+D" (Buffer), "+c" (Count)
    : "d"(Port)
    );
}

static __inline__ __attribute__((always_inline)) void __outbytestring(uint16_t Port,uint8_t *Buffer,unsigned long Count)
{
  __asm__ __volatile__ (
    "rep ; outsb"
    : "+S" (Buffer), "+c" (Count)
    : "d"(Port)
    );
}

static __inline__ __attribute__((always_inline)) void __outwordstring(uint16_t Port,uint16_t *Buffer,unsigned long Count)
{
  __asm__ __volatile__ (
    "rep ; outsw"
    : "+S" (Buffer), "+c" (Count)
    : "d"(Port)
  );
}

static __inline__ __attribute__((always_inline)) void __outdwordstring(uint16_t Port,unsigned long *Buffer,unsigned long Count)
{
  __asm__ __volatile__ (
   "rep ; outsl"
   : "+S" (Buffer), "+c" (Count)
   : "d"(Port)
   );
}

static __inline__ __attribute__((always_inline)) unsigned long __readdr0(void)
{
  unsigned long value;
  __asm__ __volatile__ (
    "mov %%dr0, %[value]"
    : [value] "=r" (value)
    );
  return value;
}

static __inline__ __attribute__((always_inline)) unsigned long __readdr1(void)
{
  unsigned long value;
  __asm__ __volatile__ (
    "mov %%dr1, %[value]"
    : [value] "=r" (value)
    );
  return value;
}

static __inline__ __attribute__((always_inline)) unsigned long __readdr2(void)
{
  unsigned long value;
  __asm__ __volatile__ (
    "mov %%dr2, %[value]"
    : [value] "=r" (value)
    );
  return value;
}

static __inline__ __attribute__((always_inline)) unsigned long __readdr3(void)
{
  unsigned long value;
  __asm__ __volatile__ (
    "mov %%dr3, %[value]"
    : [value] "=r" (value)
    );
  return value;
}

static __inline__ __attribute__((always_inline)) unsigned long __readdr7(void)
{
  unsigned long value;
  __asm__ __volatile__ (
    "mov %%dr7, %[value]"
    : [value] "=r" (value)
    );
  return value;
}

static __inline__ __attribute__((always_inline)) unsigned long __readdr(unsigned long reg)
{
  switch (reg){
    case 0:
      return __readdr0 ();
      break;

    case 1:
      return __readdr1 ();
      break;

    case 2:
      return __readdr2 ();
      break;

    case 3:
      return __readdr3 ();
      break;

    case 7:
      return __readdr7 ();
      break;

    default:
      return -1;
  }
}

static __inline__ __attribute__((always_inline)) void __writedr0(unsigned long Data)
{
  __asm__ __volatile__ (
    "mov %0, %%dr0"
    :
    : "r" (Data)
    );
}

static __inline__ __attribute__((always_inline)) void __writedr1(unsigned long Data)
{
  __asm__ __volatile__ (
    "mov %0, %%dr1"
    :
    : "r" (Data)
    );
}

static __inline__ __attribute__((always_inline)) void __writedr2(unsigned long Data)
{
  __asm__ __volatile__ (
    "mov %0, %%dr2"
    :
    : "r" (Data)
    );
}

static __inline__ __attribute__((always_inline)) void __writedr3(unsigned long Data)
{
  __asm__ __volatile__ (
    "mov %0, %%dr3"
    :
    : "r" (Data)
    );
}

static __inline__ __attribute__((always_inline)) void __writedr7(unsigned long Data)
{
  __asm__ __volatile__ (
    "mov %0, %%dr7"
    :
    : "r" (Data)
    );
}

static __inline__ __attribute__((always_inline)) void __writedr(unsigned long reg, unsigned long Data)
{
  switch (reg){
    case 0:
      __writedr0 (Data);
      break;

    case 1:
      __writedr1 (Data);
      break;

    case 2:
      __writedr2 (Data);
      break;

    case 3:
      __writedr3 (Data);
      break;

    case 7:
      __writedr7 (Data);
      break;

    default:
      ;
  }
}

static __inline__ __attribute__((always_inline)) unsigned long __readcr0(void)
{
  unsigned long value;
  __asm__ __volatile__ (
    "mov %%cr0, %[value]"
    : [value] "=r" (value));
  return value;
}

static __inline__ __attribute__((always_inline)) unsigned long __readcr2(void)
{
  unsigned long value;
  __asm__ __volatile__ (
    "mov %%cr2, %[value]"
    : [value] "=r" (value));
  return value;
}

static __inline__ __attribute__((always_inline)) unsigned long __readcr3(void)
{
  unsigned long value;
  __asm__ __volatile__ (
    "mov %%cr3, %[value]"
    : [value] "=r" (value));
  return value;
}

static __inline__ __attribute__((always_inline)) unsigned long __readcr4(void)
{
  unsigned long value;
  __asm__ __volatile__ (
    "mov %%cr4, %[value]"
    : [value] "=r" (value));
  return value;
}

static __inline__ __attribute__((always_inline)) unsigned long __readcr8(void)
{
  unsigned long value;
  __asm__ __volatile__ (
    "mov %%cr8, %[value]"
    : [value] "=r" (value));
  return value;
}

static __inline__ __attribute__((always_inline)) unsigned long __readcr(unsigned long reg)
{
  switch (reg){
    case 0:
      return __readcr0 ();
      break;

    case 2:
      return __readcr2 ();
      break;

    case 3:
      return __readcr3 ();
      break;

    case 4:
      return __readcr4 ();
      break;

    case 8:
      return __readcr8 ();
      break;

    default:
      return -1;
  }
}

static __inline__ __attribute__((always_inline)) void __writecr0(unsigned long Data)
{
  __asm__ __volatile__ (
    "mov %0, %%cr0"
    :
    : "r" (Data)
    : "memory"
    );
}

static __inline__ __attribute__((always_inline)) void __writecr2(unsigned long Data)
{
  __asm__ __volatile__ (
    "mov %0, %%cr2"
    :
    : "r" (Data)
    );
}

static __inline__ __attribute__((always_inline)) void __writecr3(unsigned long Data)
{
  __asm__ __volatile__ (
    "mov %0, %%cr3"
    :
    : "r" (Data)
    );
}

static __inline__ __attribute__((always_inline)) void __writecr4(unsigned long Data)
{
  __asm__ __volatile__ (
    "mov %0, %%cr4"
    :
    : "r" (Data)
    );
}

static __inline__ __attribute__((always_inline)) void __writecr8(unsigned long Data)
{
  __asm__ __volatile__ (
    "mov %0, %%cr8"
    :
    : "r" (Data)
    );
}

static __inline__ __attribute__((always_inline)) void __writecr(unsigned long reg, unsigned long Data)
{
  switch (reg){
    case 0:
      __writecr0 (Data);
      break;

    case 2:
      __writecr2 (Data);
      break;

    case 3:
      __writecr3 (Data);
      break;

    case 4:
      __writecr4 (Data);
      break;

    case 8:
      __writecr8 (Data);
      break;

    default:
      ;
  }
}

static __inline__ __attribute__((always_inline)) UINT64 __readmsr(UINT32 msr)
{
  UINT64 retval;
  __asm__ __volatile__(
       "rdmsr"
       : "=A" (retval)
       : "c" (msr)
       );
   return retval;
}

static __inline__ __attribute__((always_inline)) void __writemsr (UINT32 msr, UINT64 Value)
{
  __asm__ __volatile__ (
     "wrmsr"
     :
     : "c" (msr), "A" (Value)
     );
}

#if !defined(__clang__)
static __inline__ __attribute__((always_inline)) UINT64 __rdtsc(void)
{
  UINT64 retval;
  __asm__ __volatile__ (
     "rdtsc"
     : "=A" (retval));
  return retval;
}
#endif

static __inline__ __attribute__((always_inline)) void __cpuid(int CPUInfo[], const int InfoType)
{
   __asm__ __volatile__(
     "cpuid"
     :"=a" (CPUInfo[0]), "=b" (CPUInfo[1]), "=c" (CPUInfo[2]), "=d" (CPUInfo[3])
     : "a" (InfoType)
     );
}


static __inline__ __attribute__((always_inline)) void _disable(void)
{
  __asm__ __volatile__ ("cli");
}


static __inline__ __attribute__((always_inline)) void _enable(void)
{
  __asm__ __volatile__ ("sti");
}


static __inline__ __attribute__((always_inline)) void __halt(void)
{
  __asm__ __volatile__ ("hlt");
}


static __inline__ __attribute__((always_inline)) void __debugbreak(void)
{
  __asm__ __volatile__ ("int3");
}

static __inline__ __attribute__((always_inline)) void __invd(void)
{
  __asm__ __volatile__ ("invd");
}

static __inline__ __attribute__((always_inline)) void __wbinvd(void)
{
  __asm__ __volatile__ ("wbinvd");
}

static __inline__ __attribute__((always_inline)) void __lidt(void *Source)
{
  __asm__ __volatile__("lidt %0" : : "m"(*(short*)Source));
}

static __inline__ __attribute__((always_inline)) void
__writefsbyte(const unsigned long Offset, const uint8_t Data)
{
  __asm__ ("movb %[Data], %%fs:%a[Offset]"
          :
          : [Offset] "ir" (Offset), [Data] "iq" (Data));
}

static __inline__ __attribute__((always_inline)) void
__writefsword(const unsigned long Offset, const uint16_t Data)
{
  __asm__ ("movw %[Data], %%fs:%a[Offset]"
          :
          : [Offset] "ir" (Offset), [Data] "ir" (Data));
}

static __inline__ __attribute__((always_inline)) void
__writefsdword(const unsigned long Offset, const uint32_t Data)
{
  __asm__ ("movl %[Data], %%fs:%a[Offset]"
           :
           : [Offset] "ir" (Offset), [Data] "ir" (Data));
}

static __inline__ __attribute__((always_inline)) uint8_t
__readfsbyte(const unsigned long Offset)
{
  unsigned char value;
  __asm__ ("movb %%fs:%a[Offset], %[value]"
          : [value] "=q" (value)
          : [Offset] "ir" (Offset));
  return value;
}

static __inline__ __attribute__((always_inline)) uint16_t
__readfsword(const unsigned long Offset)
{
  unsigned short value;
  __asm__ ("movw %%fs:%a[Offset], %[value]"
           : [value] "=q" (value)
           : [Offset] "ir" (Offset));
  return value;
}

static __inline__ __attribute__((always_inline)) uint32_t
__readfsdword(unsigned long Offset)
{
  unsigned long value;
  __asm__ ("mov %%fs:%a[Offset], %[value]"
           : [value] "=r" (value)
           : [Offset] "ir" (Offset));
  return value;
}

#ifdef __SSE3__
typedef long long __v2di __attribute__((__vector_size__ (16)));
typedef long long __m128i __attribute__((__vector_size__ (16), __may_alias__));

static __inline__ __attribute__((always_inline)) void _mm_stream_si128_fs2 (void *__A, __m128i __B)
{
  __asm__(".byte 0x64"); // fs prefix
#if defined(__clang__)
  __builtin_nontemporal_store((__v2di)__B, (__v2di *)__A);
#else
  __builtin_ia32_movntdq ((__v2di *)__A, (__v2di)__B);
#endif
}

static __inline__ __attribute__((always_inline)) void _mm_stream_si128_fs (void *__A, void *__B)
{
  __m128i data;
  data = (__m128i) __builtin_ia32_lddqu ((char const *)__B);
  _mm_stream_si128_fs2 (__A, data);
}

static __inline__ __attribute__((always_inline)) void _mm_clflush_fs (void *__A)
{
  __asm__(".byte 0x64"); // fs prefix
  __builtin_ia32_clflush (__A);
}

#if !defined(__clang__)
static __inline __attribute__(( __always_inline__)) void _mm_mfence (void)
{
  __builtin_ia32_mfence ();
}
#else
void _mm_mfence(void);
#endif

#if !defined(__clang__)
static __inline __attribute__(( __always_inline__)) void _mm_sfence (void)
{
  __builtin_ia32_sfence ();
}
#else
void _mm_sfence(void);
#endif
#endif /* __SSE3__ */

#endif /* defined (__GNUC__) */
