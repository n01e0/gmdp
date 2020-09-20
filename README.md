# gmdp
GitHub MarkDown Previewer

## Install

```bash
cargo install gmdp
```

## Usage

```
GitHub MarkDown Previewer

USAGE:
    gmdp [FLAGS] <path>

FLAGS:
    -h, --help          Prints help information
    -o, --only_parse    without browse, only parse markdown
    -p, --port          port(default 6419)
    -V, --version       Prints version information

ARGS:
    <path>    markdown file
```

## example

```bash
$ gmdp -o README.md|w3m -T text/html
gmdp

GitHub MarkDown Previewer

Install

cargo install gmdp

Usage

GitHub MarkDown Previewer

USAGE:
    gmdp [FLAGS] <path>

FLAGS:
    -h, --help          Prints help information
    -o, --only_parse    without browse, only parse markdown
    -p, --port          port(default 6419)
    -V, --version       Prints version information

ARGS:
    <path>    markdown file
```
