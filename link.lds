ENTRY(entry_point);
SECTIONS
{
 image_start = .;
 . = 0;
 .head : {
  KEEP(*(.head))
 }
 . = 0x00010000;
 .spira : {
  KEEP(*(.spira.data))
 }
 . = 0x00010400;
 .spirah : {
  KEEP(*(.spirah.data))
 }
 . = (0x00010000 + 0x800);
 .procin.data : {
  KEEP(*(.procin.data))
 }
 . = (0x00010000 + 0x1000);
 .mdst : {
  KEEP(*(.mdst.data))
 }
 . = (0x00010000 + 0x1800);
 .cpuctrl : {
  KEEP(*(.cpuctrl.data))
 }
 . = ALIGN(0x10);
 _stext = .;
  .text : {
  *(.text*)
  *(.sfpr)
 }
 _etext = .;
 .rodata : {
  __rodata_start = .;
  *(.rodata .rodata.*)
  __rodata_end = .;
 }
 . = ALIGN(0x10);
 .init : {
  __ctors_start = .;
  KEEP(*(.ctors))
  KEEP(*(SORT(.init_array.*)))
  KEEP(*(.init_array))
  __ctors_end = .;
 }
 . = ALIGN(0x10);
 .opd : {
  *(.opd)
 }
 . = ALIGN(0x100);
 .got : {
  __toc_start = . + 0x8000;
  *(.got)
  *(.toc)
 }
 . = ALIGN(0x10);
 .opal_table : {
  __opal_table_start = .;
  KEEP(*(.opal_table))
  __opal_table_end = .;
 }
 .platforms : {
  __platforms_start = .;
  KEEP(*(.platforms))
  __platforms_end = .;
 }
 .dynsym : { *(.dynsym) }
 .dynstr : { *(.dynstr) }
 . = ALIGN(0x10);
 .dynamic : {
  __dynamic_start = .;
  *(.dynamic)
  __dynamic_end = .;
 }
 . = ALIGN(0x10);
 .rela.dyn : {
  __rela_dyn_start = .;
  *(.rela*)
  __rela_dyn_end = .;
 }
 .hash : { *(.hash) }
 .dynsym : { *(.dynsym) }
 .dynstr : { *(.dynstr) }
 . = ALIGN(0x10);
 .sym_map : {
  __sym_map_start = . ;
  KEEP(*(.sym_map))
  __sym_map_end = . ;
 }
 _romem_end = .;
 .data : {
  . = ALIGN(0x1000);
  *(.data.memcons);
  . = ALIGN(0x1000);
  *(.data.boot_trace);
  . = ALIGN(0x1000);
  *(.data*)
  *(.force.data)
  *(.toc1)
  *(.branch_lt)
 }
 . = 0x300000;
 _sbss = .;
 .bss : {
  *(.bss*)
 }
 . = ALIGN(0x10000);
 _ebss = .;
 _end = .;
       . = ALIGN(0x10000);
       .builtin_kernel : {
  __builtin_kernel_start = .;
  KEEP(*(.builtin_kernel))
  __builtin_kernel_end = .;
 }
 /DISCARD/ : {
  *(.comment)
  *(.interp)
  *(.note*)
 }
}
