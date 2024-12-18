use core::arch::{asm, global_asm};

global_asm!(
    // no "c" here, the same as riscv-rt
    ".attribute arch, \"rv64im\"",
    ".section .start, \"ax\"
     .global _start

_start:
    //.word 0x00000000
    // init all registers
    li gp, 0
    // li sp, 0
    li tp, 0
    li t0, 0
    li t1, 0
    li t2, 0
    li s0, 0
    li s1, 0
    li a0, 0
    li a1, 0
    li a2, 0
    li a3, 0
    li a4, 0
    li a5, 0
    li a6, 0
    li a7, 0
    li s2, 0
    li s3, 0
    li s4, 0
    li s5, 0
    li s6, 0
    li s7, 0
    li s8, 0
    li s9, 0
    li s10, 0
    li s11, 0
    li t3, 0
    li t4, 0
    li t5, 0
    li t6, 0

    .option push
    .option norelax

    la gp, __global_pointer$
    .option pop

    la t1, __stack_start__
    addi sp, t1, -16

    // bss must be zeroed, ddr is dirty
    la  	t1, sbss
    la   	t2, ebss
1:  bgeu 	t1, t2, 1f
    sd   	zero, 0(t1)
    addi 	t1, t1, 8
    j    	1b
1:

    la t0, _start_trap_rust
    csrw mtvec, t0
    csrw mie, zero

    li    t0, 0x00001800
    csrw  mstatus, t0

    call _setup_interrupts

    call _start_rust"
);

#[link_section = ".trap"]
#[no_mangle]
unsafe extern "riscv-interrupt-m" fn _start_trap_rust() {
    // riscv::delay::McycleDelay::new(CPU0_CORE_CLK).delay_ms(1000);

    let mepc = riscv::register::mepc::read();
    let mtval = riscv::register::mtval::read();

    let mcause = riscv::register::mcause::read();

    if mcause.is_exception() && mcause.code() == 2 {
        // Illegal instruction
        crate::ruacpu::FAULT_FLAG = true;

        if mtval & 0b11 != 0x11 {
            // C extension
            riscv::register::mepc::write(mepc + 4); // skip 2 C opcode
        } else {
            riscv::register::mepc::write(mepc + 4);
        }

        return;
    }
    println!("trap!");

    println!("mstatus: {:016x}", riscv::register::mstatus::read().bits());
    println!("mcause:  {:016x}", mcause.bits());
    println!("mtval:   {:016x}", mtval);
    println!("mepc:    {:016x}", mepc);

    loop {}
}

#[no_mangle]
unsafe extern "C" fn _setup_interrupts() {
    {
        use riscv::register::*;

        mstatus::set_mie(); // enable global interrupt
                            // mstatus::set_sie(); // and supervisor interrupt
        mie::set_mext(); // and external interrupt
        mie::set_msoft(); // and software interrupt
        mie::set_mtimer(); // and timer interrupt

        mcounteren::set_cy(); // enable cycle counter
        mcounteren::set_tm(); // and time counter

        // FPU init
        mstatus::set_fs(mstatus::FS::Initial);
        asm!("csrwi fcsr, 0");

        // Vector
        mstatus::set_vs(mstatus::VS::Initial);
    }
}
