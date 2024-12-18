use core::arch::asm;

pub unsafe fn rvv_demo1() {
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
pub unsafe fn rvv_demo2() {
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
pub unsafe fn rvv_demo3() {
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
