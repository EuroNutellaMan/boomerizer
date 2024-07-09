# Boomerizer

## Introduction
Boomerizer is a simple CLI tool that uses Rust to turn normal text into boomer text.

It randomly adds dots and emojis after words in a sentence. You can configure the percentages at the start.

```
This is.....an.... example of a...boomerized....text🤪🤪🤪 generated....😏 with boomerizer
```

## Installation

### Pre-compiled binaries
Precompile binaries are available in the [releases](https://github.com/EuroNutellaMan/boomerizer/releases) page for:
- Linux x86_64
- Windows x86_64

### Build from source
1. You will need to install rust and cargo on your machine. You can find out how to do that [here](https://doc.rust-lang.org/stable/book/ch01-01-installation.html)

2. Clone this repository wherever you want
```
git clone https://github.com/EuroNutellaMan/boomerizer.git && cd boomerizer
```

3. Build it
```
cargo build --release
```

4. You'll find your binary in:
```
target/releases/boomerizer # Linux and MacOS
target\releases\boomerizer.exe # Windows
