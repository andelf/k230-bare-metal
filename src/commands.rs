use core::{arch::asm, ptr};

pub fn handle_command_line(line: &str) {
    // println!("DEBUG: {:?}", line);

    let mut it = line.split_whitespace();

    let first = it.next();

    match first {
        Some("help") => {
            println!("Available commands:");
            println!("  help - print this help");
            println!("  echo <text> - print <text>");
            println!("  reboot - reboot the system");
            println!("  mem_read <address> <length> - read memory");
            println!("  mem_write <address> <u32> - write memory");
            println!("  tsensor - read temperature sensor");
            println!("  cpuid - print CPUID");
            println!("  serialboot - enter serial boot mode");
        }
        Some("jump") => {
            let jump_addr = it.next().and_then(parse_number).unwrap_or(0x0100_0000);

            println!("Jump to 0x{:08x}", jump_addr);
            unsafe {
                core::arch::asm!(
                    "jr {0}",
                    in(reg) jump_addr,
                    options(noreturn)
                );
            }
        }
        Some("jumpbig") => {
            let jump_addr = it.next().and_then(parse_number).unwrap_or(0x0100_0000);

            println!("Jump to 0x{:08x} with CPU1", jump_addr);
            unsafe {
                ptr::write_volatile(0x91102104 as *mut u32, jump_addr as u32);
                ptr::write_volatile(0x9110100c as *mut u32, 0x10001000);
                ptr::write_volatile(0x9110100c as *mut u32, 0x10001);
                ptr::write_volatile(0x9110100c as *mut u32, 0x10000);
            }
        }
        Some("echo") => {
            for word in it {
                print!("{} ", word);
            }
            println!("");
        }
        Some("cpuid") => {
            cpuid();
        }
        Some("reboot") => {
            println!("Reset SOC...");
            //const SOC_GLB_RST: u32 = 0x91100000 + 0x2000 + 0x60;
            //unsafe { core::ptr::write_volatile(SOC_GLB_RST as *mut u32, 0x10001) };
            pac::BOOT.soc_glb_rst().modify(|w| *w |= 0x10001);

            // not working
            //pac::RMU.cpu0_rst_ctl().write_value(0x0001); // cpu0_reset_req
        }
        Some("tsensor") => loop {
            let r = pac::TSENSOR.tsen_r().read();
            if r.ts_dout_valid() {
                let temp = r.ts_dout();
                println!("Temperature Sensor: {}", temp);
                break;
            }
        },
        Some("serialboot") => {
            crate::boot::litex_term_serial_boot();
        }
        Some("mem_read") => {
            let address = it.next();
            let length = it.next();

            match (
                address.and_then(parse_number),
                length.and_then(parse_number),
            ) {
                (Some(address), Some(length)) => {
                    if length % 4 != 0 {
                        println!("Length must be a multiple of 4");
                        return;
                    }

                    println!(
                        "Dump memory from 0x{:08x} to 0x{:08x}",
                        address,
                        address + length
                    );

                    // dump as hexdump format:
                    // 00100020  35 71 b7 2c 83 cd d9 4a  fb 8e 54 8d 00 00 00 00  |5q.,...J..T.....|

                    let mut ptr = address as *const u32;
                    let end = (address + length) as *const u32;

                    // align to 16 bytes
                    if address % 16 != 0 {
                        let aligned_address = address & !0xf;
                        let offset = (address - aligned_address) / 4;

                        print!("\n{:08x}  | ", ptr as u64);
                        for _ in 0..offset {
                            print!("         ");
                        }
                    }

                    while ptr < end {
                        if (ptr as u32) % 16 == 0 {
                            print!("\n{:08x}  | ", ptr as u32);
                        }

                        let value = unsafe { ptr.read_volatile() };
                        print!("{:08x} ", value);

                        ptr = unsafe { ptr.add(1) };
                    }

                    println!("");
                }
                _ => {
                    println!("mem_read <address> <length>");
                }
            }
        }
        Some("mem_write") => {
            let address = it.next();
            let value = it.next();

            match (address.and_then(parse_number), value.and_then(parse_number)) {
                (Some(address), Some(value)) => {
                    println!("Write 0x{:08x} to 0x{:08x}", value, address);
                    let ptr = address as *mut u32;
                    unsafe { ptr.write_volatile(value as u32) };
                }
                _ => {
                    println!("mem_write <address> <u32>");
                }
            }
        }
        Some(_) => {
            println!("Unknown command. Type 'help' for help.");
        }
        None => {
            println!("Empty command. Type 'help' for help.");
        }
    }
}

pub fn parse_number(s: &str) -> Option<u64> {
    if s.starts_with("0x") || s.starts_with("0X") {
        u64::from_str_radix(&s[2..], 16).ok()
    } else if s.starts_with("0b") || s.starts_with("0B") {
        u64::from_str_radix(&s[2..], 2).ok()
    } else {
        s.parse().ok()
    }
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
}
