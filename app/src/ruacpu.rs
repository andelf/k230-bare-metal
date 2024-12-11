//! https://github.com/nihui/ruapu/blob/master/ruapu.h

use core::arch::asm;

unsafe fn inst_i() {
    // add a0,a0,a0
    asm!(
        ".word 0x00a50533",
        out("a0") _,
    )
}

unsafe fn inst_m() {
    // addi a0,x0,2
    // mul a0,a0,a0
    // div a0,a0,a0
    asm!(
        ".word 0x00200513
         .word 0x02a50533
         .word 0x02a54533
        ", out("a0") _,
    )
}

unsafe fn inst_a() {
    // lr.w t0,(sp)
    // sc.w t0,t0,(sp)
    asm!(
        ".word 0x100122af
         .word 0x185122af",
        out("t0") _,
    )
}

unsafe fn inst_f() {
    // fmul.s fa0,fa0,fa0
    asm!(".word 0x10a57553",)
}

unsafe fn inst_d() {
    // fmul.d fa0,fa0,fa0
    asm!(".word 0x12a57553",)
}

unsafe fn inst_c() {
    // add a0,a0,a0
    // nop
    asm!(
        ".word 0x0001952a",
        out("a0") _,
    )
}

unsafe fn inst_zba() {
    // sh1add a0,a0,a0
    asm!(".word 0x20a52533",
        out("a0") _,
    )
}

unsafe fn inst_zbb() {
    // sext.b a0,a0,a0
    asm!(".word 0x60451513",
        out("a0") _,
    )
}

unsafe fn inst_zbc() {
    // clmulr a0,a0,a0
    asm!(".word 0x0aa52533",
        out("a0") _,
    )
}

unsafe fn inst_zbs() {
    // bclr a0,a0,a0
    asm!(".word 0x48a51533",
        out("a0") _,
    )
}

unsafe fn inst_zbkb() {
    // pack a0,a0,a0
    asm!(".word 0x08a54533",
        out("a0") _,
    )
}

unsafe fn inst_zbkc() {
    // clmulh a0,a0,a0
    asm!(".word 0x0aa53533",
        out("a0") _,
    )
}

unsafe fn inst_zbkx() {
    // xperm.n a0,a0,a0
    asm!(".word 0x28a52533",
        out("a0") _,
    )
}

unsafe fn inst_zcb() {
    // c.not a0 c.not a0
    asm!(".word 0x9d759d75",
        out("a0") _,
    )
}

unsafe fn inst_zfa() {
    // fli.s ft0, min
    asm!(".word 0xf0108053",
        out("ft0") _,
    )
}

unsafe fn inst_zfbfmin() {
    // fcvt.bf16.s ft0,ft0
    asm!(".word 0x44807053",
        out("ft0") _,
    )
}

unsafe fn inst_zfh() {
    // fadd.hs ft0, ft0, ft0
    asm!(".word 0x04007053",
        out("ft0") _,
    )
}

unsafe fn inst_zfhmin() {
    // fmv.x.h a0, ft0
    asm!(".word 0xe4000553",
        out("a0") _,
        out("ft0") _,
    )
}

unsafe fn inst_zicond() {
    // czero.eqz a0,a0,a0
    asm!(".word 0x0ea55533",
        out("a0") _,
    )
}

unsafe fn inst_zicsr() {
    // csrr a0, time
    asm!(".word 0xc0102573",
        out("a0") _,
    )
}

unsafe fn inst_zifencei() {
    // fence.i
    asm!(".word 0x0000100f")
}

unsafe fn inst_zmmul() {
    // mul a0,a0,a0
    asm!(".word 0x02a50533",
        out("a0") _,
    )
}

// Zk: Zkn Zks Zkr Zkne Zknd Zknh Zksed Zksh Zkt

// NIST Suite: Hash Function Instructions
unsafe fn inst_zknh() {
    // sha256sig0 a0, a1
    asm!(".word 0x10259513",
        out("a0") _,
    )
}

// Zksh - ShangMi Suite: SM3 Hash Function Instructions
unsafe fn inst_zksh() {
    // sm3p0 a0, a1
    asm!(".word 0x10859513",
        out("a0") _,
    )
}

// Zksed - ShangMi Suite: SM4 Block Cipher Instructions
unsafe fn inst_zksed() {
    // sm4ed a0, a1, a2, 3
    asm!(".word 0xf0c58533",
        out("a0") _,
    )
}

// = T-Head extensions

unsafe fn inst_xtheadba() {
    // th.addsl a0,a0,a0,#0
    asm!(".word 0x00a5150b",
        out("a0") _,
    )
}

unsafe fn inst_xtheadbb() {
    // th.srri a0,a0,#0
    asm!(".word 0x1005150b",
        out("a0") _,
    )
}

unsafe fn inst_xtheadbs() {
    // th.tst a0,a0,#0
    asm!(".word 0x8805150b",
        out("a0") _,
    )
}

unsafe fn inst_theadcmo() {
    // dcache.call
    asm!(".word 0x0020000b")
}

unsafe fn inst_xtheadcondmov() {
    // th.mveqz a0,a0,a0
    asm!(".word 0x40a5150b",
        out("a0") _,
    )
}

unsafe fn inst_xtheadfmemidx() {
    // th.flrw a0,sp,a0,#0
    asm!(".word 0x40a1650b",
        out("a0") _,
    )
}

unsafe fn inst_xtheadfmv() {
    // th.fmv.x.hw a0,fa0
    asm!(".word 0xc005150b",
        out("a0") _,
    )
}

unsafe fn inst_xtheadmac() {
    // th.mula a0,a0,a0
    asm!(".word 0x20a5150b",
        out("a0") _,
    )
}

unsafe fn inst_xtheadmemidx() {
    // th.lbia a0,(sp),#0,#0
    asm!(".word 0x1801450b",
        out("a0") _,
    )
}

unsafe fn inst_xtheadmempair() {
    // th.lwd a0,a0,(sp),#0,3
    asm!(".word 0xe0a1450b",
        out("a0") _,
    )
}

unsafe fn inst_xtheadsync() {
    // th.sync
    asm!(".word 0x0180000b")
}

unsafe fn inst_xtheadvdot() {
    // th.vmaqa.vv v0,v0,v0
    asm!(".word 0x8000600b",)
}

unsafe fn inst_spacemitvmadot() {
    // vmadot v2,v0,v0
    asm!(".word 0xe200312b",)
}

unsafe fn inst_spacemitvmadotn() {
    // vmadot3 v2,v0,v1
    asm!(".word 0xe600b12b",)
}

unsafe fn inst_spacemitvfmadot() {
    // vfmadot v2,v0,v0
    asm!(".word 0xea00012b",)
}

unsafe fn inst_xwchc() {
    // qk.c.sbsp s0, 0(sp) x2
    asm!("
        mv t0, s0
        .word 0x80408040
        mv s0, t0",
        out("t0") _,
    )
}

pub static mut FAULT_FLAG: bool = false;

pub const RUA_CPU_FEATURES: &[(&str, unsafe fn())] = &[
    ("i", inst_i),
    ("m", inst_m),
    ("a", inst_a),
    ("f", inst_f),
    ("d", inst_d),
    ("d", inst_c),
    ("zba", inst_zba),
    ("zbb", inst_zbb),
    ("zbc", inst_zbc),
    ("zbs", inst_zbs),
    ("zbkb", inst_zbkb),
    ("zbkc", inst_zbkc),
    ("zbkx", inst_zbkx),
    ("zcb", inst_zcb),
    ("zfa", inst_zfa),
    ("zfbfmin", inst_zfbfmin),
    ("zfh", inst_zfh),
    ("zfhmin", inst_zfhmin),
    ("zicond", inst_zicond),
    ("zicsr", inst_zicsr),
    ("zifencei", inst_zifencei),
    ("zmmul", inst_zmmul),
    // Zk: Zkn Zks Zkr Zkne Zknd Zknh Zksed Zksh Zkt
    // Zks: zbkb zbkc zbkx zksed zksh
    ("zknh", inst_zknh),
    ("zksh", inst_zksh),
    ("zksed", inst_zksed),
    //
    ("xtheadba", inst_xtheadba),
    ("xtheadbb", inst_xtheadbb),
    ("xtheadbs", inst_xtheadbs),
    ("xtheadcmo", inst_theadcmo), // new
    ("xtheadcondmov", inst_xtheadcondmov),
    ("xtheadfmemidx", inst_xtheadfmemidx),
    ("xtheadfmv", inst_xtheadfmv),
    ("xtheadmac", inst_xtheadmac),
    ("xtheadmemidx", inst_xtheadmemidx),
    ("xtheadmempair", inst_xtheadmempair),
    ("xtheadsync", inst_xtheadsync),
    ("xtheadvdot", inst_xtheadvdot),
    // spacemit extensions
    ("spacemitvmadot", inst_spacemitvmadot),
    ("spacemitvmadotn", inst_spacemitvmadotn),
    ("spacemitvfmadot", inst_spacemitvfmadot),
    // wch
    ("xwchc", inst_xwchc),
    // TODO: Andes
    // - Andes V5 Performance Extension
    // - Andes Code Dense (CoDense) Extension
    // - Andes INT4 Vector Load Extension
    // - Andes Scalar BFLOAT16 Conversion Extension
    // - Andes Vector BFLOAT16 Conversion Extension
    // - Andes V5 Vector Packed FP16 extension
    // - Andes V5 Vector Dot Product extension
    // - Andes V5 Vector Small INT Handling extension
    // - Andes Vector Quad-Widening Integer Multiply-Add extension
    // - (Deprecated) Andes Half-Precision Floating-Point Extension
    // - (Deprecated) FSHW (Floating-point Store to Half-precision from Single-precision)
    // - AndeStar V5 DSP ISA Extension
];

unsafe fn detect_vlen() -> i32 {
    // read csr 0xc22
    let mut vlenb = 0;
    asm!("csrr {0}, 0xc22", out(reg) vlenb);

    vlenb * 8
}

pub fn detect() {
    unsafe {
        for (name, inst) in RUA_CPU_FEATURES.iter() {
            FAULT_FLAG = false;
            inst();
            if FAULT_FLAG {
                println!("{}: no", name);
            } else {
                println!("{}: yes", name);
            }
        }

        let vlen = detect_vlen();
        if vlen > 0 {
            println!("zvl{}b: yes", vlen);
        }
    }
    println!("Hello, world!");
}
