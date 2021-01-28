# Detox
[![Crates.io](https://img.shields.io/crates/v/detox.svg)](https://crates.io/crates/detox)
[![Build Status](https://img.shields.io/github/workflow/status/whitfin/detox/CI)](https://github.com/whitfin/detox/actions)

Detox is a very small CLI tool used to clean up development directories to
save disk space. This is particularly handy to use prior to backing up your
development machine. It was written as a personal tool, but figured it might
be useful to others.

The main aim is to reduce the amount of space in project directories without
specifically altering the projects. This pretty much means the basics, such
as removing target directories, dependency directories, compressing version
control trees, etc.

The list of structures supported is as below. If you want to add support for
a new structure, feel free; the initial list is pretty much based on what I
have on my development machines on a daily basis.

* Cargo (Rust)
* Git
* Gradle (Java)
* Maven (Java)
* Mix (Elixir)
* Node.js

If this is the first time you have run this tool, please do read the source
to validate that it's not going to wipe something important. It should be
fairly safe given that it only looks at build files etc, but know that you
are running it at your own risk!

## Installation

Detox is written in Rust, and is available for download via the repository
on [crates.io](https://crates.io/crates/detox). The easiest way to get this
at this point is to install it via Cargo:

```shell
$ cargo install detox
```

## Usage

The CLI is tiny and instructions are provided via the documentation:

```shell
$ detox -h
$ detox <location>
```

Locations are checked recursively for files which might signal a development
directory. Various files which are "unnecessary" will then be stripped away,
and the output will tell you how much space you saved.
