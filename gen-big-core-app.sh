#!/bin/bash

set -ex

cargo objcopy -p app --release --target riscv64gcv-unknown-none-elf.json -- -O binary big-core.bin

ls -lah big-core.bin
