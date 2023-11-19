# chptr

Rust application to print Unicode code point values, UTF-8 hex bytes, and binary representation of the UTF-8 hex bytes for text strings.

This is just a little utility written while learning Rust, nothng fancy.

## Installation

The easiest, assuming you have Rust and Cargo installed, is to just download the source code and `cargo build`.

## Usage

The tool can read a string from `stdin` or an argument.

### Reading from `stdin`

```shell
(base) ╭─alan@WokeAF
╰─➤  echo "✳Моя собака." | chptr
✳ | U+2733 | E2 9C B3     | 1110 00101001 11001011 0011
М | U+041C | D0 9C        | 1101 00001001 1100
о | U+043E | D0 BE        | 1101 00001011 1110
я | U+044F | D1 8F        | 1101 00011000 1111
  | U+0020 | 20           | 0010 0000
с | U+0441 | D1 81        | 1101 00011000 0001
о | U+043E | D0 BE        | 1101 00001011 1110
б | U+0431 | D0 B1        | 1101 00001011 0001
а | U+0430 | D0 B0        | 1101 00001011 0000
к | U+043A | D0 BA        | 1101 00001011 1010
а | U+0430 | D0 B0        | 1101 00001011 0000
. | U+002E | 2E           | 0010 1110
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


