#define Efer  0xC0000080
#define Lme (1<<8)

#define Pse 0x10
#define Pge 0x40
#define Pae 0x20
#define PgePae 0x60

#define PE 1       //Protected Mode Enable         If 1, system is in protected mode, else system is in real mode
#define MP 2       //Monitor co-processor  Controls interaction of WAIT/FWAIT instructions with TS flag in CR0
#define EM 4       //Emulation     If set, no x87 floating-point unit present, if clear, x87 FPU present
#define TS 8       //Task switched         Allows saving x87 task context upon a task switch only after x87 instruction used
#define ET 0x10       //Extension type        On the 386, it allowed to specify whether the external math coprocessor was an 80287 or 80387
#define NE 0x20       //Numeric error         Enable internal x87 floating point error reporting when set, else enables PC style x87 error detection
#define WP 0x10000      //Write protect         When set, the CPU can't write to read-only pages when privilege level is 0
#define AM 0x40000      //Alignment mask        Alignment check enabled if AM set, AC flag (in EFLAGS register) set, and privilege level is 3
#define NW 0x20000000     //Not-write through     Globally enables/disable write-through caching
#define CD 0x40000000      //Cache disable         Globally enables/disable the memory cache
#define PG 0x80000000      //Paging        If 1, enable paging and use the § CR3 register, else disable paging.
#define CDNWTSMP 0x6000000a

#define NUM_FIXED_RANGES                88
#define RANGES_PER_FIXED_MTRR           8
#define MTRR_FIX_64K_00000              0x250
#define MTRR_FIX_16K_80000              0x258
#define MTRR_FIX_16K_A0000              0x259
#define MTRR_FIX_4K_C0000               0x268
#define MTRR_FIX_4K_C8000               0x269
#define MTRR_FIX_4K_D0000               0x26a
#define MTRR_FIX_4K_D8000               0x26b
#define MTRR_FIX_4K_E0000               0x26c
#define MTRR_FIX_4K_E8000               0x26d
#define MTRR_FIX_4K_F0000               0x26e
#define MTRR_FIX_4K_F8000               0x26f
#define MTRR_CAP_MSR			0x0fe	
//#define MTRR_PHYS_BASE(reg)		(0x200 + 2 * (reg))
//#define MTRR_PHYS_MASK(reg)		(MTRR_PHYS_BASE(reg) + 1)
#define MTRR_DEF_TYPE_MSR		0x2ff
#define MTRR_DEF_TYPE_MASK		0xff
#define MTRR_DEF_TYPE_EN		(1 << 11)
#define MTRR_DEF_TYPE_FIX_EN		(1 << 10)

#define CR0_PE          (1 <<  0)
#define CR0_MP          (1 <<  1)
#define CR0_EM          (1 <<  2)
#define CR0_TS          (1 <<  3)
#define CR0_ET          (1 <<  4)
#define CR0_NE          (1 <<  5)
#define CR0_WP          (1 << 16)
#define CR0_AM          (1 << 18)
#define CR0_NW          (1 << 29)
#define CR0_CD          (1 << 30)
#define CR0_PG          (1 << 31)

/* PML4E/PDPE/PDE/PTE */
#define PteP		0x0000000000000001	/* Present */
#define PteRW		0x0000000000000002	/* Read/Write */
#define PteU		0x0000000000000004	/* User/Supervisor */
#define PtePWT		0x0000000000000008	/* Page-Level Write Through */
#define PtePCD		0x0000000000000010	/* Page Level Cache Disable */
#define PteA		0x0000000000000020	/* Accessed */
#define PteD		0x0000000000000040	/* Dirty */
#define PtePS		0x0000000000000080	/* Page Size */
#define Pte4KPAT	PtePS			/* PTE PAT */
#define PteG		0x0000000000000100	/* Global */
#define Pte2MPAT	0x0000000000001000	/* PDE PAT */
#define Pte1GPAT	Pte2MPAT		/* PDPE PAT */
#define PteNX		0x8000000000000000	/* No Execute */
