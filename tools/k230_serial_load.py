#!/usr/bin/env python3
"""Load a binary into the running K230 bare-metal shell over UART."""

from __future__ import annotations

import argparse
import sys
import time
from pathlib import Path

import serial


SFL_MAGIC_REQ = b"sL5DdSMmkekro\n"
SFL_MAGIC_ACK = b"z6IHG7cYDID6o\n"

SFL_CMD_ABORT = 0x00
SFL_CMD_LOAD = 0x01
SFL_CMD_JUMP = 0x02

SFL_ACK_SUCCESS = b"K"
SFL_ACK_CRCERROR = b"C"
SFL_ACK_UNKNOWN = b"U"
SFL_ACK_ERROR = b"E"

ANSI_CPR_QUERY = b"\x1b[6n"
ANSI_CPR_RESPONSE = b"\x1b[24;80R"


def crc16(data: bytes) -> int:
    crc = 0
    for byte in data:
        x = ((crc >> 8) & 0xFF) ^ byte
        x ^= x >> 4
        crc = ((crc << 8) ^ (x << 12) ^ (x << 5) ^ x) & 0xFFFF
    return crc


def make_frame(command: int, payload: bytes) -> bytes:
    if len(payload) > 255:
        raise ValueError("SFL payload must be 255 bytes or less")

    body = bytes([command]) + payload
    return bytes([len(payload)]) + crc16(body).to_bytes(2, "big") + body


def read_until(
    port: serial.Serial,
    needle: bytes,
    timeout: float,
    echo: bool,
    answer_cpr: bool,
) -> bytes:
    deadline = time.monotonic() + timeout
    data = bytearray()
    scan_offset = 0

    while time.monotonic() < deadline:
        chunk = port.read(max(1, port.in_waiting))
        if chunk:
            data.extend(chunk)
            if echo:
                sys.stdout.buffer.write(chunk)
                sys.stdout.buffer.flush()
            if answer_cpr:
                while True:
                    query_at = data.find(ANSI_CPR_QUERY, scan_offset)
                    if query_at < 0:
                        scan_offset = max(0, len(data) - len(ANSI_CPR_QUERY) + 1)
                        break
                    port.write(ANSI_CPR_RESPONSE)
                    port.flush()
                    scan_offset = query_at + len(ANSI_CPR_QUERY)
            if needle in data:
                return bytes(data)
        else:
            time.sleep(0.01)

    tail = bytes(data[-200:])
    raise TimeoutError(f"timed out waiting for {needle!r}; last bytes: {tail!r}")


def service_terminal(port: serial.Serial, duration: float, echo: bool, answer_cpr: bool) -> None:
    deadline = time.monotonic() + duration
    data = bytearray()
    scan_offset = 0

    while time.monotonic() < deadline:
        chunk = port.read(max(1, port.in_waiting))
        if chunk:
            data.extend(chunk)
            if echo:
                sys.stdout.buffer.write(chunk)
                sys.stdout.buffer.flush()
            if answer_cpr:
                while True:
                    query_at = data.find(ANSI_CPR_QUERY, scan_offset)
                    if query_at < 0:
                        scan_offset = max(0, len(data) - len(ANSI_CPR_QUERY) + 1)
                        break
                    port.write(ANSI_CPR_RESPONSE)
                    port.flush()
                    scan_offset = query_at + len(ANSI_CPR_QUERY)
        else:
            time.sleep(0.01)


def read_ack(port: serial.Serial, timeout: float) -> bytes:
    deadline = time.monotonic() + timeout
    while time.monotonic() < deadline:
        reply = port.read(1)
        if reply:
            return reply
    raise TimeoutError("timed out waiting for SFL ACK")


def send_frame(port: serial.Serial, command: int, payload: bytes, ack_timeout: float, retries: int) -> None:
    frame = make_frame(command, payload)

    for attempt in range(retries + 1):
        port.write(frame)
        port.flush()
        reply = read_ack(port, ack_timeout)

        if reply == SFL_ACK_SUCCESS:
            return
        if reply == SFL_ACK_CRCERROR and attempt < retries:
            continue

        names = {
            SFL_ACK_CRCERROR: "CRC error",
            SFL_ACK_UNKNOWN: "unknown command",
            SFL_ACK_ERROR: "device error",
        }
        reason = names.get(reply, f"unexpected reply {reply!r}")
        raise RuntimeError(f"SFL frame failed: {reason}")


def enter_serialboot(
    port: serial.Serial,
    command: str,
    timeout: float,
    echo: bool,
    answer_cpr: bool,
) -> None:
    if command:
        port.write(b"\r\n")
        port.flush()
        service_terminal(port, 0.5, echo, answer_cpr)
        port.write(command.encode("ascii") + b"\r\n")
        port.flush()

    read_until(port, SFL_MAGIC_REQ, timeout, echo, answer_cpr)
    port.write(SFL_MAGIC_ACK)
    port.flush()


def upload(
    port: serial.Serial,
    image: Path,
    address: int,
    frame_size: int,
    ack_timeout: float,
    retries: int,
) -> int:
    data = image.read_bytes()
    if frame_size < 5 or frame_size > 255:
        raise ValueError("--frame-size must be between 5 and 255")

    chunk_size = frame_size - 4
    total = len(data)
    start = time.monotonic()

    print(f"Uploading {image} to 0x{address:08x} ({total} bytes)")

    offset = 0
    while offset < total:
        chunk = data[offset : offset + chunk_size]
        payload = (address + offset).to_bytes(4, "big") + chunk
        send_frame(port, SFL_CMD_LOAD, payload, ack_timeout, retries)
        offset += len(chunk)

        percent = 100 * offset // total if total else 100
        sys.stdout.write(f"\r{offset:8d}/{total} bytes {percent:3d}%")
        sys.stdout.flush()

    elapsed = max(time.monotonic() - start, 1e-6)
    print(f"\nUpload complete ({total / elapsed / 1024:.1f} KiB/s)")
    return total


def monitor(port: serial.Serial) -> None:
    print("Monitoring serial output. Press Ctrl-C to exit.")
    while True:
        chunk = port.read(max(1, port.in_waiting))
        if chunk:
            sys.stdout.buffer.write(chunk)
            sys.stdout.buffer.flush()
        else:
            time.sleep(0.01)


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("port", help="serial port, for example /dev/cu.usbmodem7E238F0602BE1")
    parser.add_argument("--image", default="big-core.bin", type=Path, help="binary image to upload")
    parser.add_argument("--address", default="0x01000000", help="load and jump address")
    parser.add_argument("--baudrate", "--speed", default=115200, type=int, help="serial baudrate")
    parser.add_argument("--command", default="serialboot", help="shell command that enters SFL mode")
    parser.add_argument("--timeout", default=5.0, type=float, help="seconds to wait for SFL magic")
    parser.add_argument("--ack-timeout", default=3.0, type=float, help="seconds to wait for frame ACK")
    parser.add_argument("--frame-size", default=64, type=int, help="SFL payload size including 4-byte address")
    parser.add_argument("--retries", default=3, type=int, help="CRC retry count per frame")
    parser.add_argument("--no-jump", action="store_true", help="upload only")
    parser.add_argument("--monitor", action="store_true", help="print serial output after jump")
    parser.add_argument("--quiet", action="store_true", help="hide bootloader text while waiting for magic")
    parser.add_argument("--answer-cpr", action="store_true", help="answer ANSI cursor position queries")
    return parser.parse_args()


def main() -> int:
    args = parse_args()
    image = args.image
    address = int(args.address, 0)

    if not image.exists():
        raise FileNotFoundError(image)

    with serial.Serial(args.port, args.baudrate, timeout=0.02, write_timeout=3) as port:
        enter_serialboot(port, args.command, args.timeout, not args.quiet, args.answer_cpr)
        upload(port, image, address, args.frame_size, args.ack_timeout, args.retries)

        if args.no_jump:
            print("Leaving SFL mode")
            send_frame(port, SFL_CMD_ABORT, b"", args.ack_timeout, args.retries)
        else:
            print(f"Jumping to 0x{address:08x}")
            send_frame(port, SFL_CMD_JUMP, address.to_bytes(4, "big"), args.ack_timeout, args.retries)

        if args.monitor:
            monitor(port)

    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except KeyboardInterrupt:
        raise SystemExit(130)
