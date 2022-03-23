mmu_get:
	mrc p15, 0, r0, c1, c0, 0
	mov pc, lr

mmu_set:
	mcr     p15, 0, r0, c1, c0, 0
	mov pc, lr
