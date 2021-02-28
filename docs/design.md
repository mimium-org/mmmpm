# mmmpm - specification design

In this document we try to design and describe what is a mimium package and how mmmpm is used.

**The contents shall be changed by discussions**

## Packages

The term *package* denotes a set of files with metadata. The purpose of mimium packages is to construct mimium libraries or applications or songs as reusable, distributable and manageable format.

Usually, packages are directory arranged we specify and hosted as like GitHub repository. Future, packages are archived like .tar.gz or any zipped formats.

It may be good that each package introduces its (package name) namespace because avoiding name confliction between packages another.

### Package structure

Packages must obey *mimium package structure* specified here.

First, packages must contain the `mmmp.toml`. This contains package metadata. For details see the section "mmmp.toml".

Second, packages must contain one or more mimium source files. The source files shall directly place at package root or separated directories.

Third, optionally, packages shall contain some *assets*; sound files or score data or any files needed to perform the package.

Here we show example package structure:

```
mmm-package-example
├── README.md
├── kick.wav
├── main.mmm
├── mmmp.toml
└── util.mmm
```

### `mmmp.toml`

In this secion we describe the `mmmp.toml` file. This contains package metadata like these below:

- metadata
    - package name (namespase name?)
    - author information
    - version information
- package information
    - library or application or song?
        - entrypoint filename (this file has `dsp()` in it)
    - dependency information (to other mimium packages)
        - with early version, mmmpm command only can clone from Git repository
    - source files or directory where source files are placed
    - asset files (.wav or any other files allowed by mimium)

### `mmmp.toml` example

I tenporary design the contents of `mmmp.toml` like this (this example is written with refferencing Rust's `Cargo.toml`):

```
[metadata]
name = "mmm-package-example"
version = "0.0.1"
authors = ["hoge <hoge@example.com>"]
license = "ISC"

[dependencies]
example-oscillators = { git = "https://example.com/ociellators.git", version = "1.0.4" }
# Future, create our central package repository...?
# filters = "1.0.0"

[package]
type = "app"
entrypoint = "main.mmm"
source_dir = "/"
asset_dir = "/"
```

### Discussion

- consideration about package type (library, application, song or any other?)
- `mmmp.toml` or other name?
- format for the package description file: toml, JOSN or other format?
- fixed filename for the entrypoint? cf. in Rust, `main.rs`
- fixed sources and assets directory name? cf. in Rust, `src`

## `mmmpm` command

```
Usage: mmmpm COMMAND [OPTIONS]

Commands:
    install PACKAGE           Install a package and its dependencies
    list                      List all packages installed
    run PACKAGE               Run (an application or a song) package specified as PACKAGE or PATH

Options:
    -V, --version             Print version information
    -v, --verbose             Prints what mmmpm does
    -h, --help                Prints help information
```

where the `PACKAGE` is an package specifier; this includes package name itself, package path or package URL like remote Git repository.

### Duscussion

- `mmm` command as like `go` command or `cargo` command. I thought it may be good idea :)
