# aki-mcycle

*aki-mcycle* is the program that mark up text with cycling color.

## Features

*aki-mcycle*  mark up text with cycling color.

* command help

```text
aki-mcolor --help
```

```text
Usage:
  aki-mcycle [options]

mark up text with cycling color.

Options:
  -e, --exp <exp>      regex (default: ' ([0-9A-Z]{3,}):')

  -H, --help     display this help and exit
  -V, --version  display version information and exit

Env:
  RUST_CYCLE_COLOR_RED_ST      red start sequence
  RUST_CYCLE_COLOR_GREEN_ST    green start sequence
  RUST_CYCLE_COLOR_BLUE_ST     blue start sequence
  RUST_CYCLE_COLOR_CYAN_ST     cyan start sequence
  RUST_CYCLE_COLOR_MAGENDA_ST  magenda start sequence
  RUST_CYCLE_COLOR_YELLOW_ST   yellow start sequence
  RUST_CYCLE_COLOR_ED          color end sequence
```

* minimum support rustc 1.38.0

## Quick install

1. you can install this into cargo bin path:

```
cargo install aki-mcycle
```

2. you can build debian package:

```
cargo deb
```

and install **.deb** into your local repository of debian package.

## Examples

#### Command line example 1

Extract "`arm`" from the rustup target list and make "`linux-[^ ]+`" **color**.

- 1st match: makes '`linux-musl`' **red**
- 2nd match: makes '`linux-musleabi`' **green**
- 3rd match: makes '`linux-musleabihf`' **blue**
- 4th match: makes '`linux-muslabi64`' **cyan**

```
rustup target list | aki-mline -e arm | aki-mcycle -e "linux-[^ ]+"
```

result output :

![out rustup image]

[out rustup image]: https://raw.githubusercontent.com/aki-akaguma/aki-mcycle/main/img/out-rustup-1.png

- [aki-mline](https://crates.io/crates/aki-mline): extract match line command like grep.

#### Library example

See [`fn execute()`] for this library examples.

[`fn execute()`]: crate::execute
