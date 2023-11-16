# chptr

Rust application to print Unicode code point values and UTF-8 hex bytes for text strings

This is just a little utility written while learning Rust, nothng fancy.

## Installation

The easiest, assuming you have Rust and Cargo installed, is to just download the source code and `cargo build`.

## Usage

The tool can read a string from `stdin` or an argument.

### Reading from `stdin`

```shell
(base) ╭─alan@WokeAF
╰─➤  echo "моя собака." | chptr
м | U+043C | D0 BC
о | U+043E | D0 BE
я | U+044F | D1 8F
  | U+0020 | 20
с | U+0441 | D1 81
о | U+043E | D0 BE
б | U+0431 | D0 B1
а | U+0430 | D0 B0
к | U+043A | D0 BA
а | U+0430 | D0 B0
. | U+002E | 2E
```

### From an argument

```shell
(base) ╭─alan@WokeAF
╰─➤  chptr "うちの犬"
う | U+3046 | E3 81 86
ち | U+3061 | E3 81 A1
の | U+306E | E3 81 AE
犬 | U+72AC | E7 8A AC
```


