# Winx86_64-Selfinjection-Agent

A small Rust project for learning and testing **self-injection and shellcode execution concepts on Windows x86_64**.

The shellcode is placed directly inside `src/main.rs`.

> **Note:** This project is intended for authorized security research and isolated lab environments only.

## How It Works

The basic workflow is:

```text
Shellcode
    ↓
src/main.rs
    ↓
Cargo Build
    ↓
Windows x86_64 .exe
```

The shellcode is stored as bytes inside `main.rs`. Change the embedded payload there whenever you want to test a different payload.

## Requirements

You need:

* Rust + Cargo
* `x86_64-pc-windows-gnu` Rust target
* MinGW-w64 when cross-compiling from Linux

Add the Windows target with:

```bash
rustup target add x86_64-pc-windows-gnu
```

On Debian based Distros:

```bash
sudo apt install mingw-w64
```

## Build

Build a debug version:

```bash
cargo build --target x86_64-pc-windows-gnu
```

Build an optimized release version:

```bash
cargo build --release --target x86_64-pc-windows-gnu
```

The release executable will be generated in:

```text
target/x86_64-pc-windows-gnu/release/
```

## Useful Commands

Check the project:

```bash
cargo check --target x86_64-pc-windows-gnu
```

Format the code:

```bash
cargo fmt
```

Clean previous builds:

```bash
cargo clean
```

## Payload Architecture

The project targets **Windows x86_64**, so the shellcode should also be suitable for a 64-bit Windows environment.

## Testing

Use a Windows VM or another isolated test environment when experimenting with the project. This makes it much easier to monitor the behavior and restore the system if something goes wrong.

## Disclaimer

This project is for **educational purposes, Red Team Operations Knowledge, malware analysis, and authorized security research**.
Only test it on systems where you have explicit permission.
