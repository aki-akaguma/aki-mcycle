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

mark up text with the cyclic color.

Options:
  -e, --exp <exp>   write it in the cyclic color (default: ' ([0-9A-Z]{3,}):')

  -H, --help        display this help and exit
  -V, --version     display version information and exit

Option Parameters:
  <exp>     regular expression, color the entire match with the cyclic color.

Environments:
  AKI_MCYCLE_COLOR_SEQ_RED_ST       red start sequence specified by ansi
  AKI_MCYCLE_COLOR_SEQ_GREEN_ST     green start sequence specified by ansi
  AKI_MCYCLE_COLOR_SEQ_BLUE_ST      blue start sequence specified by ansi
  AKI_MCYCLE_COLOR_SEQ_CYAN_ST      cyan start sequence specified by ansi
  AKI_MCYCLE_COLOR_SEQ_MAGENDA_ST   magenda start sequence specified by ansi
  AKI_MCYCLE_COLOR_SEQ_YELLOW_ST    yellow start sequence specified by ansi
  AKI_MCYCLE_COLOR_SEQ_ED           color end sequence specified by ansi
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

## Changelogs

[This crate's changelog here.](https://github.com/aki-akaguma/aki-mcycle/blob/main/CHANGELOG.md)
