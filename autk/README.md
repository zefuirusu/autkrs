# AUTK: Auditors' Toolkit
## Description
`AUTK` is a cli toolkit designed for auditors, published by GNU/GPL version 3.0.

>Note that `AUTK` has not come into a stable version yet.
>Use it at your own risk.

## Install
`AUTK` is developed by rust, which can be installed by `cargo build --release`.
## Usage

``` sh
# autk --help
Auditors\' Toolkit.

Usage: autk <COMMAND>

Commands:
  config  config....
  show    read info from Excel file.
  help    Print this message or the help of the given subcommand(s)

Options:
  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```

### Config project

``` sh
# autk config  --help
config....

Usage: autk config <COMMAND>

Commands:
  new
  help  Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```

### Show data

``` sh
# autk show --help
read info from Excel file.

Usage: autk show <COMMAND>

Commands:
  shape  show shape of sheets in the file.
  sht    show the whole expected sheet.
  row    show the target row data.
  col    show the target column data.
  match  match specific column by regular expression.
  help   Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```

## Develop tools
1. [rust toolchain](https://rust-lang.org/)
2. [Doom Emacs](https://github.com/doomemacs/doomemacs.git)
