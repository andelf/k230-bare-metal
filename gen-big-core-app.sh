#!/bin/bash

set -ex

cargo -Zjson-target-spec build -p app --release --target riscv64gcv-unknown-none-elf.json --locked
rust-objcopy -O binary target/riscv64gcv-unknown-none-elf/release/app big-core.bin

ls -lah big-core.bin
