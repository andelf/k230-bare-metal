#!/bin/bash

set -ex

cargo objcopy --release --target riscv64gc-unknown-none-elf.json -- -O binary firmware.bin && python genimage.py

