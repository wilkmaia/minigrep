# minigrep
[![Build Status](https://travis-ci.org/wilkmaia/minigrep.svg?branch=master)](https://travis-ci.org/wilkmaia/minigrep)
[![codecov](https://codecov.io/gh/wilkmaia/minigrep/branch/master/graph/badge.svg)](https://codecov.io/gh/wilkmaia/minigrep)
[![License:MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

minigrep is a minimal implementation of the `grep` unix tool implemented in Rust.
It should mirror `grep`'s basic functionalities.

## Building

### Current development branch

To build the current development branch:

```sh
$ git clone https://github.com/wilkmaia/minigrep.git
$ cd minigrep
$ cargo build # "cargo build --release" for a release build
```

### Latest release

To build the latest release:

```sh
$ curl -sSL -o minigrep-latest.tar.gz https://github.com/wilkmaia/minigrep/archive/0.1.0.tar.gz
$ tar -zxf minigrep-latest.tar.gz
$ cd minigrep-0.1.0
$ cargo build --release
```

## Usage

Usage aims to be similar to Unix `grep`.

### Current supported usage methods

1. Input file
```sh
minigrep <pattern> <filepath>
```
`pattern` is currently a string-only. minigrep should accept regular expressions as well in the [near future](https://github.com/wilkmaia/minigrep/issues/13).
