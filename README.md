
# k230-bare-metal

Bare-metal Rust Embedded on K230.

Currently at a very early stage - contributions, discussions, and further investigations are warmly welcomed.
It's mainly focused on understanding the booting internals through code analysis and reverse-engineering.

Status: Experimental 🧪

Note: This is a preliminary research project. All findings and documentation are based on personal investigation and may need verification.

Boards:

- CanMV-LCKFB
- CanMV-K230 V1P0 - modify `src/ddr_init/mod.rs` for different DDR configurations

## Usage

Generate TF card image:

```console
cargo objcopy --release -- -O binary firmware.bin && python genimage.py
```

Then flash `firmware.img` to a TF card.

Now you can boot the K230 board with the TF card. A simple command line interface is available on UART0.

Generate CPU1 (big-core) firmware image:

```console
> ./gen-big-core-app.sh
+ cargo objcopy -p app --release -- -O binary big-core.bin
    Finished `release` profile [optimized] target(s) in 0.04s
+ ls -lah big-core.bin
-rwxr-xr-x  1 mono  staff    17K Dec 10 18:46 big-core.bin
```

Install [LiteX](https://github.com/enjoy-digital/litex) toolset (`litex_term`). Then run:

```console
> litex_term /dev/tty.usbmodem56C40035621 --kernel-adr 0x01000000 --kernel ./big-core.bin
.... (booting log)
Press Q or ESC to abort boot completely.
sL5DdSMmkekro
[LITEX-TERM] Received firmware download request from the device.
[LITEX-TERM] Uploading ./big-core.bin to 0x01000000 (17400 bytes)...
[LITEX-TERM] Upload calibration... (inter-frame: 640.00us, length: 64)
[LITEX-TERM] Upload complete (9.9KB/s).
[LITEX-TERM] Booting the device.
[LITEX-TERM] Done.
Jumping to 0x01000000...

K230> jumpbig
```

For a running `K230>` shell, use the repository loader:

```console
> python3 tools/k230_serial_load.py /dev/cu.usbmodem7E238F0602BE1 --image ./big-core.bin --address 0x01000000 --monitor
Booting from serial...
Press Q or ESC to abort boot completely.
sL5DdSMmkekro
Uploading big-core.bin to 0x01000000
Upload complete
Jumping to 0x01000000
```

The loader sends `serialboot`, waits for the LiteX SFL magic, uploads the image, and sends
the SFL `JUMP` command. The firmware starts CPU1 from the uploaded address.
CPU1 console output is forwarded through a mailbox-style shared console and printed by CPU0
on UART0, so `--monitor` shows the second-stage output on the same serial port.

### PMU GPIO and K230D Lite RGB LED

K230D exposes `GPIO64..GPIO71` through the PMU/RTC domain. The firmware maps them as
GPIO group2 on `GPIO1` port1:

```text
GPIO64 -> GPIO1 port1 bit0
GPIO65 -> GPIO1 port1 bit1
GPIO66 -> GPIO1 port1 bit2
GPIO71 -> GPIO1 port1 bit7
```

On LushanPi Lite K230D, the onboard RGB LED is wired to:

```text
red   -> GPIO65 / INT1
green -> GPIO66 / INT2
blue  -> GPIO71 / OUT1
```

The shell exposes PMU/GPIO inspection and LED commands:

```console
K230> pmu_dump
K230> pin_dump 65
K230> pin_dump 66
K230> pin_dump 71
K230> led test
K230> embassy_led 2 120
```

`led test` drives red, green, blue, white, and off in sequence. This has been verified
on K230D hardware. The PMU GPIO command path is restricted to the schematic-backed LED
pins `65`, `66`, and `71`; normal-domain `gpio_set` is limited to `GPIO0..GPIO63`.

`embassy_led [cycles] [delay_ms]` runs the same RGB LED sequence through
`embassy_time::Timer`. The current hardware test used `embassy_led 2 120`; the shell
returned `Embassy RGB LED pattern done`, and the MachineTimer interrupt path reported
`Embassy time driver handled alarm` for each awaited delay.

### Boot Log on UART0

```text
 _  __              _            _
| |/ /___ _ __   __| |_ __ _   _| |_ ___
| ' // _ \ '_ \ / _` | '__| | | | __/ _ \
| . \  __/ | | | (_| | |  | |_| | ||  __/
|_|\_\___|_| |_|\__,_|_|   \__, |\__\___|
                           |___/

Booting K230 using Rust ....
0000000d: PMU Major Msg: End of CA training
00000000: PMU Major Msg: End of initialization
00000002: PMU Major Msg: End of read enable training
00000001: PMU Major Msg: End of fine write leveling
0000000a: PMU Major Msg: End of read dq deskew training
000000fd: PMU Major Msg: End of MPR read delay center optimization
000000fe: PMU Major Msg: End of Write leveling coarse delay
00000004: PMU Major Msg: End of write delay center optimization
00000003: PMU Major Msg: End of read delay center optimization
00000009: PMU Major Msg: End of max read latency training
00000007: PMU Major Msg: Firmware run has completed
DDR init done!
misa: 800000000094112f
  RV64ABCDFIMSUX
mcycle: 1177686900
```


## Firmware Image Format

### For TF Card

```text
00100000  4b 32 33 30 8c fc 02 00  00 00 00 00 bf 8d 0f 38  |K230...........8| <- Firmware header
          ^           ^            ^           ^
          +  "K230"   + Length     |           + SHA256 hash
                                   + Encryption 0: non encryption, 1 SM4, 2 AES+RSA

00100010  03 f3 87 07 fa 1b d8 1d  4f a0 cd a0 7b 54 35 bd  |........O...{T5.|  <- SHA256 hash cont.
00100020  35 82 85 89 66 4d ac 27  ca f8 56 49 00 00 00 00  |5...fM.'..VI....|  <- SHA256 hash cont. + Padding
00100030  00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00  |................|  <- Padding
*

00100210  00 00 00 00 73 25 40 f1  2a 82 ae 84 93 01 00 00  |....s%@.*.......|  <- Firmware data
          ^           ^
          |           + Firmware code, unencrypted, (RV64 assembly)
          + Firmware version
```

Firmware is loaded to 0x80300000, not 0x80200000. 0x80200000 to 0x802fffff is reserved for the bootloader(BootRom).

## About K230

### Peripherals

The peripheral IP cores are:

- UART: DW_apb_uart
- SPI: DW_apb_ssi
- I2C: DW_apb_i2c
- GPIO: DW_apb_gpio
- I2S: DW_apb_i2s
- Timer: DW_apb_timers
- Watchdog: DW_apb_wdt
