
# k230-bare-metal

Doc: TODO

Boards:

- CanMV-K230 V1P0
- CanMV-LCKFB

## Boot Log

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
          ^ "K230"    ^            ^           ^
                      + Length     |           + SHA256 hash
                                   + Encryption 0: non encryption, 1 SM4, 2 AES+RSA

00100010  03 f3 87 07 fa 1b d8 1d  4f a0 cd a0 7b 54 35 bd  |........O...{T5.|  <- SHA256 hash cont.
00100020  35 82 85 89 66 4d ac 27  ca f8 56 49 00 00 00 00  |5...fM.'..VI....|  <- SHA256 hash cont. + Padding
00100030  00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00  |................|
*
00100210  00 00 00 00 73 25 40 f1  2a 82 ae 84 93 01 00 00  |....s%@.*.......|  <- Firmware data
          ^           ^
          |           + Firmware code, unencrypted, (RV64 assembly)
          + Firmware version
```
