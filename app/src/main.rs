#![no_std]
#![no_main]
#![feature(abi_riscv_interrupt)]

use core::arch::asm;
use core::arch::global_asm;
use core::ptr;
use embedded_hal::delay::DelayNs;
use pac::UART0;

#[macro_export]
macro_rules! println {
    ($($arg:tt)*) => {
        {
            use core::fmt::Write;
            writeln!(&mut $crate::Console, $($arg)*).unwrap();
        }
    };
    () => {
        {
            use core::fmt::Write;
            writeln!(&mut $crate::Console, "").unwrap();
        }
    };
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        {
            use core::fmt::Write;
            write!(&mut $crate::Console, $($arg)*).unwrap();
        }
    };
}

pub mod ruacpu;

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
        ruacpu::FAULT_FLAG = true;

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

#[derive(Debug)]
pub struct Console;

impl core::fmt::Write for Console {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for c in s.as_bytes() {
            unsafe {
                while !UART0.lsr().read().thre() {
                    asm!("nop");
                }

                UART0.thr().write(|w| w.set_thr(*c));
            }
        }

        Ok(())
    }
}

unsafe fn rvv_demo1() {
    let mut out = 0;

    let a: [u16; 130] = [
        1000, 1001, 1002, 1003, 1004, 1005, 1006, 1007, 1008, 1009, 1010, 1011, 1012, 1013, 1014,
        1015, 1016, 1017, 1018, 1019, 1020, 1021, 1022, 1023, 1024, 1025, 1026, 1027, 1028, 1029,
        1030, 1031, 1032, 1033, 1034, 1035, 1036, 1037, 1038, 1039, 1040, 1041, 1042, 1043, 1044,
        1045, 1046, 1047, 1048, 1049, 1050, 1051, 1052, 1053, 1054, 1055, 1056, 1057, 1058, 1059,
        1060, 1061, 1062, 1063, 1064, 1065, 1066, 1067, 1068, 1069, 1070, 1071, 1072, 1073, 1074,
        1075, 1076, 1077, 1078, 1079, 1080, 1081, 1082, 1083, 1084, 1085, 1086, 1087, 1088, 1089,
        1090, 1091, 1092, 1093, 1094, 1095, 1096, 1097, 1098, 1099, 1100, 1101, 1102, 1103, 1104,
        1105, 1106, 1107, 1108, 1109, 1110, 1111, 1112, 1113, 1114, 1115, 1116, 1117, 1118, 1119,
        1120, 1121, 1122, 1123, 1124, 1125, 1126, 1127, 0x0000, 0x0000,
    ];
    let b: [u16; 130] = [
        32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54,
        55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77,
        78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 91, 92, 93, 94, 95, 96, 97, 98, 99,
        100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111, 112, 113, 114, 115, 116, 117,
        118, 119, 120, 121, 122, 123, 124, 125, 126, 127, 128, 129, 130, 131, 132, 133, 134, 135,
        136, 137, 138, 139, 140, 141, 142, 143, 144, 145, 146, 147, 148, 149, 150, 151, 152, 153,
        154, 155, 156, 157, 158, 159, 0x0000, 0x0000,
    ];
    let mut c: [u16; 140] = [0; 140];

    asm!("
        vsetvli a3, a0, e32, m8, ta, ma
        mv {ret}, a3
        ", ret = out(reg) out,
    );
    println!("out: {:08x}", out);
    let mut vl = 0;
    asm!("
1:
        vsetvli t0, a4, e16, m8, ta, ma
        slli t1, t0, 1 // multiply # elements this iteration by 2 bytes/source element

        vle16.v v0, (a0)
        vle16.v v8, (a1)
        vadd.vv v16, v0, v8
        vse16.v v16, (a2)

        add a0, a0, t1 // add # elements to pointer
        add a1, a1, t1
        add a2, a2, t1

        sub a4, a4, t0 // desc elements
        bnez a4, 1b

        mv {vl}, t0

    ", vl = out(reg) vl,
        in("a0") &a as *const _ as u64,
        in("a1") &b as *const _ as u64,
        in("a2") &mut c as *mut _ as u64,
        in("a4") 128,
        in("t0") 0,
    );

    for i in 0..140 {
        print!("c[{:02x}]: {:05}", i, c[i]);
        if i % 8 == 7 {
            println!();
        } else {
            print!(" ");
        }
    }

    println!("vl: 0x{:08x} {}", vl, vl);
}

// Load and store 64-bit vector registers
unsafe fn rvv_demo2() {
    let vregs: [u128; 32] = [0xffffffffffffffffffffffffffffffff; 32];

    // 128 / 8 = 16
    asm!("
        vsetvli x0, a1, e64, m1, ta, ma

        vle64.v v0, (a0)
        addi a0, a0, 16
        vle64.v v1, (a0)
        addi a0, a0, 16
        vle64.v v2, (a0)
        addi a0, a0, 16
        vle64.v v3, (a0)
        addi a0, a0, 16
        vle64.v v4, (a0)
        addi a0, a0, 16
        vle64.v v31, (a0)

        ",
        in("a0") vregs.as_ptr() as u64,
        in("a1") 2,
    );

    let mut vregs: [u128; 32] = [0x0; 32];
    asm!(
        "
        vsetvli x0, a1, e64, m1, ta, ma
        vse64.v v0, (a0)
        addi a0, a0, 16
        vse64.v v1, (a0)
        addi a0, a0, 16
        vse64.v v2, (a0)
        addi a0, a0, 16
        vse64.v v3, (a0)
        addi a0, a0, 16
        vse64.v v4, (a0)
        addi a0, a0, 16
        vse64.v v5, (a0)
        addi a0, a0, 16
        vse64.v v6, (a0)
        addi a0, a0, 16
        vse64.v v7, (a0)
        addi a0, a0, 16
        vse64.v v8, (a0)
        addi a0, a0, 16
        vse64.v v9, (a0)
        addi a0, a0, 16
        vse64.v v10, (a0)
        addi a0, a0, 16
        vse64.v v11, (a0)
        addi a0, a0, 16
        vse64.v v12, (a0)
        addi a0, a0, 16
        vse64.v v13, (a0)
        addi a0, a0, 16
        vse64.v v14, (a0)
        addi a0, a0, 16
        vse64.v v15, (a0)
        addi a0, a0, 16
        vse64.v v16, (a0)
        addi a0, a0, 16
        vse64.v v17, (a0)
        addi a0, a0, 16
        vse64.v v18, (a0)
        addi a0, a0, 16
        vse64.v v19, (a0)
        addi a0, a0, 16
        vse64.v v20, (a0)
        addi a0, a0, 16
        vse64.v v21, (a0)
        addi a0, a0, 16
        vse64.v v22, (a0)
        addi a0, a0, 16
        vse64.v v23, (a0)
        addi a0, a0, 16
        vse64.v v24, (a0)
        addi a0, a0, 16
        vse64.v v25, (a0)
        addi a0, a0, 16
        vse64.v v26, (a0)
        addi a0, a0, 16
        vse64.v v27, (a0)
        addi a0, a0, 16
        vse64.v v28, (a0)
        addi a0, a0, 16
        vse64.v v29, (a0)
        addi a0, a0, 16
        vse64.v v30, (a0)
        addi a0, a0, 16
        vse64.v v31, (a0)
        ",
        in("a0") vregs.as_mut_ptr() as u64,
        in("a1") 2, // at least 2 element, or elase no RVV registers will be enabled
        // using 1 only enables Lower half of each register
    );

    for i in 0..32 {
        println!("v{:<2}: 0x{:032x}", i, vregs[i]);
    }
}

// compute a * b + c
unsafe fn rvv_demo3() {
    let mut a: [f32; 1024] = [0.0; 1024];
    let mut b: [f32; 1024] = [0.0; 1024];
    let mut c: [f32; 1024] = [0.0; 1024];

    for i in 0..1024 {
        a[i] = i as f32;
        b[i] = 1024.0 - i as f32;
        c[i] = 2.33 * i as f32;
    }

    // let mut out: [f32; 1024] = [0.0; 1024];

    let mut batch = 0;
    asm!(
        "
1:
        vsetvli t1, a0, e32, m8, ta, ma
        slli t2, t1, 2 // multiply # elements this iteration by 4 bytes/source element
        mv t0, t1

        vle32.v v0, (a1)
        vle32.v v8, (a2)
        vle32.v v16, (a3)

        // vfmacc.vv vd, vs1, vs2, vm    # vd[i] = +(vs1[i] * vs2[i]) + vd[i]
        vfmacc.vv v16, v0, v8

        vse32.v v16, (a3) // save result to c

        add a1, a1, t2 // add # elements to pointer
        add a2, a2, t2
        add a3, a3, t2

        sub a0, a0, t1 // desc elements
        bnez a0, 1b
    ",
        in("a0") 1024,
        in("a1") &a as *const _ as u64,
        in("a2") &b as *const _ as u64,
        in("a3") &mut c as *mut _ as u64,
        out("t0") batch,
        out("t1") _,
        out("t2") _,
    );

    // 32 * 32 = 1024 = 128 * 8 using 8 vector registers, each 128 bits
    println!("batch: {}", batch); // batch = 32

    for i in 0..8 {
        print!("c[{:04x}]: {:05}", i, c[i]);
        if i % 8 == 7 {
            println!();
        } else {
            print!(" ");
        }
    }
}

fn init_uart() {
    let r = UART0;

    r.thr().write(|w| w.set_thr(b'H' as u8));
    r.thr().write(|w| w.set_thr(b'H' as u8));
    r.thr().write(|w| w.set_thr(b'H' as u8));
    r.thr().write(|w| w.set_thr(b'H' as u8));
    r.thr().write(|w| w.set_thr(b'H' as u8));

    unsafe {
        ptr::write_volatile(0x911050c8 as *mut u32, 0x00000a8f); // UART3_TX. GPIO50
        ptr::write_volatile(0x911050cc as *mut u32, 0x80000bd0); // UART3_RX
    }

    let baud_rate = 115200;
    let clock_in = 50_000_000;
    let div = clock_in / (16 * baud_rate);

    // set baudrate
    r.lcr().write(|w| w.set_dlab(true));
    r.dlh().write(|w| w.set_dlh((div >> 8) as u8));
    r.dll().write(|w| w.set_dll(div as u8));
    r.lcr().write(|w| w.set_dlab(false));

    r.lcr().write(|w| {
        w.set_stop(pac::uart::vals::StopBits::STOP1);
        w.set_wls(pac::uart::vals::DataBits::BIT8);
        w.set_pen(false);
    });
    r.fcr().write(|w| w.set_fifoe(true));
    // no modem
    r.mcr().write(|w| {
        w.set_out1(false);
        w.set_out2(false);
    });
    // no interrupt
    r.ier().modify(|w| w.0 = 0);
}

#[no_mangle]
unsafe extern "C" fn _start_rust() -> ! {
    init_uart();

    let mut delay = riscv::delay::McycleDelay::new(1_600_000_000);

    println!("\r\n\r\n2nd stage on CPU1");

    cpuid();

    // read csr 0xc22
    let mut vlenb = 0;
    asm!("csrr {0}, 0xc22", out(reg) vlenb);

    println!("vlenb: {:08x}, {}", vlenb, vlenb * 8);

    println!("-= RVV Demo 1 =-");
    rvv_demo1();

    println!("-= RVV Demo 2 =-");
    rvv_demo2();

    println!("-= RVV Demo 3 =-");
    rvv_demo3();

    println!("Rua CPU:");

    // asm!("dcache.cpa a0");

    ruacpu::detect();

    loop {
        println!(
            "!! Hello, world! {} {:016x}",
            riscv::register::mhartid::read(),
            riscv::register::mcycle::read64()
        );

        delay.delay_ms(2000);
    }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    println!("Panic: {:?}", _info);
    loop {}
}

pub fn cpuid() {
    let mstatus = riscv::register::mstatus::read();
    println!("mstatus: {:016x}", mstatus.bits());

    let mie = riscv::register::mie::read();
    println!("mie: {:016x}", mie.bits());

    let mip = riscv::register::mip::read();
    println!("mip: {:016x}", mip.bits());

    let misa = riscv::register::misa::read().unwrap();

    println!("misa: {:x}", misa.bits());
    print!("  RV64");
    for c in 'A'..='Z' {
        if misa.has_extension(c) {
            print!("{}", c);
        }
    }
    println!();

    let mvendorid = riscv::register::mvendorid::read().unwrap();
    println!("mvendorid: {:x}", mvendorid.bits());

    let marchid = riscv::register::marchid::read().unwrap();
    println!("marchid: {:x}", marchid.bits());

    let mhartid = riscv::register::mhartid::read();
    println!("mhartid: {:x}", mhartid);

    let mut cpuid0: u64;
    let mut cpuid1: u64;
    let mut cpuid2: u64;
    unsafe {
        asm!("
        csrr {0}, 0xfc0
        csrr {1}, 0xfc0
        csrr {2}, 0xfc0
    ", out(reg) cpuid0, out(reg) cpuid1, out(reg) cpuid2);
    }
    println!("cpuid: {:08x} {:08x} {:08x}", cpuid0, cpuid1, cpuid2);

    let mut mxstatus: u64;
    unsafe {
        asm!("csrr {0}, 0x7C0", out(reg) mxstatus);
    }
    println!("mxstatus: {:08x}", mxstatus);
}
