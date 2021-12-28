# bonk
![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)
![Cargo](https://img.shields.io/badge/cargo-%23000000.svg?style=for-the-badge&logo=cargo&logoColor=white)
![GitHub](https://img.shields.io/github/license/sohomdatta1/bonk?style=for-the-badge)

A tool for playing around with binary files during capture the flag challenges.

Alternatively, a fun side-project that I decided to work on to reimplement widely used tools and learn (and understand) rust along the way.

## Getting Started

```sh
git clone https://github.com/sohomdatta1/bonk
cd bonk
cargo build --release
sudo cp ./target/release/bonk /usr/local/bin/bonk
```

## Features

Operations:

- [x] `version` - print version
- [x] `str <file>` - extract all strings in the file
- [x] `hex <file>` - hexdump the file
- [x] `cut <file> <start> <end> <outfile>` - extract a section of the file
- [ ] `merge <infile1> <infile2> <outfile>` - concat two files
- [ ] `rev <file> <outfile>` - reverse the file
- [ ] `diff <file1> <file2>` - diff two files at the binary level
- [ ] `file <file>` - print file information
- [x] `asciiart` - print ascii art of the binary and it's version
- [ ] `elf <file>` - parse as elf and output useful information
- [ ] `elf dump <file> <offset>` - print x86 disassembly of the file at the given offset
