/* TODO: make an asm-common file or something */

#define r0 0
#define r1 1
#define r2 2
#define r3 3
#define r4 4
#define r5 5
#define r6 6
#define r7 7
#define r8 8
#define r29 29
#define r31 31

.section ".head","ax"

/* FIXME: process relocations */
__head:
.  = 0x0
	b entry_point

.  = 0x10
.global entry_point
entry_point:
	mr 	r31,r3
// save the FDT pointer

	lis r1,0x1000 // stick our stack at 1GB, because why not

	// figure out the run address
	bl 1f
1: 	mflr r3
        subi r3,r3,(1b - __head)

	// all the cool kids are hard coding a toc pointer
	// this is just .got + 0x8000, I need to fix the linker script at some point...
	lis 	r2,(__toc_start - __head)@ha
	addi 	r2,r2,(__toc_start - __head)@l
	add 	r2,r2,r3 // add the load offset to the TOC pointer

	// process the relocations
	addis 	r4, r3, (__rela_dyn_start - __head)@ha
	addi 	r4, r4, (__rela_dyn_start - __head)@l

	addis 	r5, r3, (__rela_dyn_end - __head)@ha
	addi 	r5, r5, (__rela_dyn_end - __head)@l

reloc:	cmpd 	r4, r5
	beq 1f

	// register usage
	// r4 - current relocation
	// r5 - end of reloc table
	// r3 - base offset
	// r6 - reloc appendend
	// r7 - offset to apply reloc on
	ld 	r7,0(r4)
	ld 	r6,16(r4)
	// FIXME: check that the reloc is R_PPC64_RELATIVE. that's all ld seems to create though
	add 	r6,r6,r3 // apply relocate
	add 	r7,r7,r3
	std 	r6,0(r7)
	addi 	r4,r4,24
	b  	reloc

1:	mr r4, r31
	bl _start

. = 0xe40
	b .

.text

.global eh_personality
eh_personality:
	blr

.global _Unwind_Resume
_Unwind_Resume:
.global stack_exhaused
stack_exhaused:
	b .

.global ohshit
ohshit:
	attn

# mmio accessors
.global inb
inb:
	lbzcix r3, 0,r3
	blr


.global outb
outb:
	stbcix r4, 0, r3
	blr
