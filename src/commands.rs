use core::arch::asm;

use embedded_hal::delay::DelayNs;

pub fn handle_command_line(line: &str) {
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
            println!("  pmu_dump - dump PMU and PMU IO registers");
            println!("  pin_dump <pin> - dump GPIO/IOMUX state");
            println!("  gpio_get <0-71> - read GPIO external level");
            println!("  gpio_set <0-63> <0|1> - set normal-domain GPIO");
            println!("  pmu_gpio_set <65|66|71> <0|1> - set PMU LED GPIO");
            println!("  pmu_out1 <0|1> - set PMU OUT1 software level");
            println!("  led <off|red|green|blue|white|test> - control K230D Lite RGB LED");
            println!("  cpuid - print CPUID");
            println!("  serialboot - enter serial boot mode");
            println!("  jump <address> - jump to address");
            println!("  jumpbig <address> - jump to address with CPU1, and set CPU0 to wfi state");
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
            crate::boot::jump_big_core(jump_addr as u32);
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
                println!("Sensor Raw: {}", temp);
                println!("Temperature: {:.2}°C", tsensor_calculate_temperature(temp));
                break;
            }
        },
        Some("pmu_dump") => {
            crate::board_gpio::dump_pmu();
        }
        Some("pin_dump") => {
            let pin = it.next().and_then(parse_u8);
            match pin.and_then(|pin| crate::board_gpio::dump_pin(pin).ok()) {
                Some(dump) => {
                    println!(
                        "pin {} group{} port{} bit{}",
                        dump.pin, dump.group, dump.port, dump.bit
                    );
                    println!("iomux/io_cfg = 0x{:08x}", dump.iomux);
                    println!("dr          = 0x{:08x}", dump.dr);
                    println!("ddr         = 0x{:08x}", dump.ddr);
                    println!("ctl         = 0x{:08x}", dump.ctl);
                    println!("ext         = 0x{:08x}", dump.ext);
                }
                None => println!("pin_dump <0-71>"),
            }
        }
        Some("gpio_get") => {
            let pin = it.next().and_then(parse_u8);
            match pin.and_then(|pin| crate::board_gpio::read_gpio(pin).ok()) {
                Some(high) => println!("{}", high as u8),
                None => println!("gpio_get <0-71>"),
            }
        }
        Some("gpio_set") => {
            let pin = it.next().and_then(parse_u8);
            let high = it.next().and_then(parse_bool);
            match (pin, high) {
                (Some(pin), Some(high)) => match crate::board_gpio::set_gpio(pin, high) {
                    Ok(()) => println!("GPIO{}={}", pin, high as u8),
                    Err(_) => println!("gpio_set supports normal-domain GPIO0-63"),
                },
                _ => println!("gpio_set <0-63> <0|1>"),
            }
        }
        Some("pmu_gpio_set") => {
            let pin = it.next().and_then(parse_u8);
            let high = it.next().and_then(parse_bool);
            match (pin, high) {
                (Some(pin), Some(high)) => match crate::board_gpio::set_pmu_led_gpio(pin, high) {
                    Ok(()) => println!("PMU GPIO{}={}", pin, high as u8),
                    Err(_) => println!("pmu_gpio_set supports LED pins 65, 66, 71"),
                },
                _ => println!("pmu_gpio_set <65|66|71> <0|1>"),
            }
        }
        Some("pmu_out1") => {
            let high = it.next().and_then(parse_bool);
            match high {
                Some(high) => {
                    crate::board_gpio::set_pmu_out1(high);
                    println!(
                        "PMU OUT1 software level={}",
                        crate::board_gpio::pmu_out1_level() as u8
                    );
                }
                None => println!("pmu_out1 <0|1>"),
            }
        }
        Some("led") => match it.next() {
            Some("off") => set_led(false, false, false),
            Some("red") => set_led(true, false, false),
            Some("green") => set_led(false, true, false),
            Some("blue") => set_led(false, false, true),
            Some("white") => set_led(true, true, true),
            Some("test") => led_test(),
            _ => println!("led <off|red|green|blue|white|test>"),
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

fn set_led(red: bool, green: bool, blue: bool) {
    match crate::board_gpio::set_rgb(red, green, blue) {
        Ok(()) => println!("LED r={} g={} b={}", red as u8, green as u8, blue as u8),
        Err(_) => println!("LED control supports pins 65, 66, 71"),
    }
}

fn led_test() {
    let mut delay = riscv::delay::McycleDelay::new(crate::CPU0_CORE_CLK);
    let sequence = [
        (true, false, false),
        (false, true, false),
        (false, false, true),
        (true, true, true),
        (false, false, false),
    ];

    for (red, green, blue) in sequence {
        set_led(red, green, blue);
        delay.delay_ms(400);
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

fn parse_u8(s: &str) -> Option<u8> {
    parse_number(s).and_then(|value| u8::try_from(value).ok())
}

fn parse_bool(s: &str) -> Option<bool> {
    match s {
        "0" | "low" | "off" => Some(false),
        "1" | "high" | "on" => Some(true),
        _ => None,
    }
}

pub fn cpuid() {
    let mstatus = riscv::register::mstatus::read();
    println!("mstatus: {:016x}", mstatus.bits());

    let mie = riscv::register::mie::read();
    println!("mie: {:016x}", mie.bits());

    let mip = riscv::register::mip::read();
    println!("mip: {:016x}", mip.bits());

    let misa = riscv::register::misa::read();

    println!("misa: {:x}", misa.bits());
    print!("  RV64");
    for c in 'A'..='Z' {
        if misa.has_extension(c) {
            print!("{}", c);
        }
    }
    println!();

    let mvendorid = riscv::register::mvendorid::read();
    println!("mvendorid: {:x}", mvendorid.bits());

    let marchid = riscv::register::marchid::read();
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

fn tsensor_calculate_temperature(data: u16) -> f64 {
    use num_traits::float::FloatCore;
    let data = data as f64;
    1e-10 * data.powi(4) * 1.01472 - 1e-6 * data.powi(3) * 1.10063 + 4.36150e-3 * data.powi(2)
        - 7.10128 * data
        + 3565.87
}
