// Copyright 2019 the u-root Authors. All rights reserved
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

;; .globl kernel_start
;; kernel_start:
;; //	.incbin "boot/zImage.boot"
;; 	.align 2
;; kernel_end:
;; 	.globl kernel_size
;; kernel_size:
;; 	.int kernel_end - kernel_start

;; 	.globl dtb_start
;; dtb_start:
;; //	.incbin "boot/platform.dtb.boot"
;; 	.align 2
;; dtb_end:
;; 	.globl dtb_size
;; dtb_size:
;; 	.int dtb_end - dtb_start

;; 	.globl initrd_start
;; initrd_start:
;; //	.incbin "boot/loader.cpio.gz"
;; 	.align 2
;; initrd_end:
;; 	.globl initrd_size
;; initrd_size:
;; 	.int initrd_end - initrd_start
