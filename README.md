<div align="center">

# 🪄 any-file-binder

[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Version](https://img.shields.io/badge/version-2.0-brightgreen.svg)](https://github.com/wHoIsDReAmer/any-file-binder/releases)

A post-modern CLI tool for secure file binding on Windows systems, written in Rust 🦀

[Features](#features) • [Installation](#installation) • [Usage](#usage) • [License](#license)

</div>

## ✨ Features

- Secure file encryption using AES-256-CBC and AES-128-CBC
- Multiple file binding support
- Custom stub executable configuration
- Zero external dependencies for runtime

## 🚀 Installation
No installation providing, just build and run.

## 📖 Usage

### Basic Command Structure
```bash
any-file-binder.exe -f <files...> [-s <stub>] [-o <output>]
```

### Command Arguments
| Argument | Description | Default |
|----------|-------------|---------|
| `-f, --files` | Files to bind (multiple allowed) | Required |
| `-s, --stub` | Custom stub executable path | `stub.exe` |
| `-o, --output` | Output file path | `output.exe` |

## 🔧 Technical Details

### Binary Structure
The binder combines multiple files into a single executable in a simple and secure way:

```
[Stub Executable][__PARSE_STUB__][Metadata][Bound Files]
```

#### Encryption Header
```
| Field           | Type | Size    | Description           |
|-----------------|------|---------|------------------------|
| Separator       | str  | varies  | "__PARSE_STUB__"      |
| Encryption Type | u8   | 1 byte  | 0: AES-256, 1: AES-128|
```

#### File Entry Structure
```
| Field              | Type   | Description            |
|--------------------|--------|------------------------|
| Extension Length   | u8    | Size of file extension |
| Extension          | string | File extension         |
| Encrypted Length   | u64    | Size of encrypted data |
| Encrypted Content  | bytes  | Encrypted file data    |
```

### Supported Encryption Types
- `0`: AES-256-CBC
- `1`: AES-128-CBC

## 📝 License
```
MIT License

Copyright (c) 2024 wHoIsDReAmer

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software for non-commercial purposes only, including the rights
to use, copy, modify, merge, publish, and distribute copies of the Software,
subject to the following conditions:

1. The Software may not be used for any commercial purposes whatsoever.
   Commercial use includes but is not limited to:
   - Using the Software in a commercial product
   - Using the Software in a business environment
   - Selling or licensing the Software
   - Using the Software to generate revenue directly or indirectly

2. The above copyright notice and this permission notice shall be included in all
   copies or substantial portions of the Software.

3. Any distribution of the Software or derivative works must be under the same
   non-commercial terms.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
```

## ⚠️ Educational Purpose Only

This project was created for educational purposes to learn about Windows PE file format and Encryption. Please do not use this tool for malware or any malicious purposes. The author takes no responsibility for misuse of this software.
