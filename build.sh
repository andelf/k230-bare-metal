#!/bin/bash


cargo objcopy -- -O binary firmware.bin && python genimage.py && ls -la firmware.img
