
# k230-bare-metal

Bare-metal Rust Embedded on K230.

Currently at a very early stage - contributions, discussions, and further investigations are warmly welcomed.
It's mainly focused on understanding the booting internals through code analysis and reverse-engineering.

Status: Experimental 🧪

Note: This is a preliminary research project. All findings and documentation are based on personal investigation and may need verification.

Boards:

- CanMV-K230 V1P0
- CanMV-LCKFB

## Boot Log on UART0

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

In `board_common.h`:

```c
#define K230_IMAGE_MAGIC_NUM 0x3033324B // "K230"

typedef enum {
  NONE_SECURITY = 0,
  GCM_ONLY,
  CHINESE_SECURITY,
  INTERNATIONAL_SECURITY
} crypto_type_e;

typedef struct __firmware_head_st {
  uint32_t magic;  // 方便升级时快速判断固件是否有效。
  uint32_t length; // 从存储介质读到SRAM的数据量
  crypto_type_e
      crypto_type; // 支持国密或国际加密算法，或支持不加密启动(otp可以控制是否支持)。
  // 设想这样一个场景，如果固件只使用对称加密，在工厂批量生产的时候，解密密钥必然会泄露给工厂。如果使用非对称加密就可以这种问题了，只需要把公钥交给工厂。
  union verify_ {
    struct rsa_ {
      uint8_t n
          [256]; // 非对称加密的验签，防止固件被篡改。同时其HASH值会被烧录到otp。
      uint32_t e;
      uint8_t signature[256];
    } rsa;
    struct sm2_ {
      uint32_t idlen;
      uint8_t id[512 - 32 * 4];
      uint8_t pukx[32];
      uint8_t puky[32];
      uint8_t r[32];
      uint8_t s[32];
    } sm2;
    struct none_sec_ {
      uint8_t signature
          [32]; // 计算HASH保证启动固件的完整性。避免程序异常难以定位原因。
      uint8_t reserved[516 - 32];
    } none_sec;
  } verify;
} __attribute__((packed, aligned(4))) firmware_head_s; //总的512+16 bytes
```

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
