MEMORY { .spl_mem : ORIGIN = 0x80300000, LENGTH = 0x80000 }
MEMORY { .bss_mem : ORIGIN = 0x80380000, LENGTH = 0x20000 }
OUTPUT_ARCH("riscv")

PROVIDE(__stack_start__ = ORIGIN(.bss_mem) + LENGTH(.bss_mem));
ENTRY(_start)
SECTIONS
{
 . = ALIGN(4);
 .text : {
  *(.start)
  *(.text*)
 } > .spl_mem

 . = ALIGN(4);
 .rodata : {
  *(SORT_BY_ALIGNMENT(SORT_BY_NAME(.rodata*)))
 } > .spl_mem

 . = ALIGN(4);
 .data : {
  __global_pointer$ = . + 0x800;
  *(.data*)
 } > .spl_mem

 . = ALIGN(4);

 .got : {
  __got_start = .;
  *(.got.plt) *(.got)
  __got_end = .;
 } > .spl_mem

 . = ALIGN(4);
 __u_boot_list : {
  KEEP(*(SORT(__u_boot_list*)));
 } > .spl_mem
 . = ALIGN(4);
 .binman_sym_table : {
  __binman_sym_start = .;
  KEEP(*(SORT(.binman_sym*)));
  __binman_sym_end = .;
 } > .spl_mem
 . = ALIGN(4);
 /DISCARD/ : { *(.rela.plt*) }
 .rela.dyn : {
  __rel_dyn_start = .;
  *(.rela*)
  __rel_dyn_end = .;
 } > .spl_mem
 . = ALIGN(4);
 .dynsym : {
  __dyn_sym_start = .;
  *(.dynsym)
  __dyn_sym_end = .;
 } > .spl_mem
 . = ALIGN(4);
 _end = .;
 _image_binary_end = .;

 .bss : {
  __bss_start = .;
  *(.bss*)
  . = ALIGN(8);
  __bss_end = .;
 } > .bss_mem
}
