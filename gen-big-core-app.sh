#!/bin/bash

set -ex

cargo objcopy -p app --release -- -O binary big-core.bin

ls -lah big-core.bin
