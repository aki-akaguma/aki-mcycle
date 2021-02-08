# aki-mcycle

*aki-mcycle* is the program that mark up text with cycling color.

## Features

*aki-mcycle*  mark up text with cycling color.

* example

command:
```
`aki-mcycle` -H
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

```
cat mail.log | aki-mcycle -e " ([0-9A-Z]{3,}):"
```
