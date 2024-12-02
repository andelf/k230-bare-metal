
# k230-bare-metal

Doc: TODO

Boards:

- CanMV-K230 V1P0
- CanMV-LCKFB

## Firmware Image Format

### For TF Card

```text
00100000  4b 32 33 30 8c fc 02 00  00 00 00 00 bf 8d 0f 38  |K230...........8|
          ^ "K230"    ^            ^           ^
                      + Length     |           + SHA256 hash
                                   + Encryption 0: non encryption, 1 SM4, 2 AES+RSA

00100010  03 f3 87 07 fa 1b d8 1d  4f a0 cd a0 7b 54 35 bd  |........O...{T5.|
00100020  35 82 85 89 66 4d ac 27  ca f8 56 49 00 00 00 00  |5...fM.'..VI....|
00100030  00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00  |................|
*
00100210  00 00 00 00 73 25 40 f1  2a 82 ae 84 93 01 00 00  |....s%@.*.......|
          ^           ^
          |           + Firmware code, unencrypted, (RV64 assembly)
          + Firmware version
```
