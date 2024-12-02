import struct
import sys
import hashlib


def sha256(message):
    digest = hashlib.sha256(message).digest()
    print("SHA256: ", digest.hex())

    return digest


VERSION = b"\x00\x00\x00\x00"
MAGIC = b"K230"

with open("./firmware.bin", "rb") as f:
    data = f.read()

intput_data = VERSION + data

data_len = len(intput_data)
raw_data_len = data_len.to_bytes(4, byteorder="little")

encryption_type = 0
encryption_type = encryption_type.to_bytes(4, byteorder="little")

hash_data = sha256(intput_data)

firmware = MAGIC + raw_data_len + encryption_type + hash_data

firmware += bytes(516 - 32)
firmware += intput_data

img = bytes(0x100000) + firmware

# fill 512 boundary
if len(img) % 512 != 0:
    img += bytes(512 - len(img) % 512)


with open("./firmware.img", "wb") as f:
    f.write(img)


print("len", len(img))
