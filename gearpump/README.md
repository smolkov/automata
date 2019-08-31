#   `gearpump`

 **📦  Gear Pump device driver writed in [🦀 **Rust**](https://github.com/lar-rs/gearpump)**


🚧 _Work In Progress_ 🚧

* **TODO: Welche Aufgaben hat es?**


 * �

[![travis build Status](https://travis-ci.com/lar-rs/gearpump.svg?branch=master)](https://travis-ci.com/lar-rs/gearpump)
<!-- [![builds.sr.ht status](https://builds.sr.ht/~asmolkov/wqa/.build.yml.svg)](https://builds.sr.ht/~asmolkov/automata/.build.yml?) -->
[![open issue]][issue]
![Minimum Rust Version][min-rust-badge]

## 🎙️ Commands

`gearpump` is a CLI tool designed for setup and read ndir sensors data.


  - ### 🦀⚙️ `driver`
    run driver and wath socket for connections.
    All of the arguments and flags to this command are optional:
        - `path`: defaults name `system`.

        Build your project. This command wath link to `ndir1` directory
        with the `"type"` declared there.

  - ### 🔧 `config`
        - `dout`: set digital output number
        - `device`: set digital device name

    ```
     config --dout=12
    ```



## 🔩 Configuration

`init` initialize directory.



## ⚓ Installation

1. Install `cargo`:

    Edinburgh is installed through [Cargo](https://github.com/rust-lang/cargo#compiling-from-source), a Rust package manager. Rustup, a tool for installing Rust, will also install Cargo. On Linux and macOS systems, `rustup` can be installed as follows:

    ```
    curl https://sh.rustup.rs -sSf | sh
    ```

    Additional installation methods are available [here](https://forge.rust-lang.org/other-installation-methods.html).

2. Install `gearpump`:

    ```
    cargo install gearpump
    ```

- **Troubleshooting** `libdbus-sys` errors

    On Ubuntu OS install

    ```
    $ sudo apt install libdbus-1-dev
    ```


<!-- Badges -->
[issue]: https://img.shields.io/github/issues/lar-rs/gearpump?style=flat-square
[min-rust-badge]: https://img.shields.io/badge/rustc-1.38+-blue.svg

<!-- Server on tide [creating 🌊 web-server .deb binary with rust](https://gi.net.in/posts/creating-web-server-deb-binary-with-rust/) -->

