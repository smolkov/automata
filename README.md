#   `automata`

 **📦  water quality analyzers rewrite in [🦀 **Rust**](https://github.com/smolkov/automata)**

* Design-UI package : :iphone:

🚧 _Work In Progress_ 🚧

* **TODO: Welche Aufgaben hat es?**


 * �

[![travis build Status](https://travis-ci.com/lar-rs/edinburgh.svg?branch=master)](https://travis-ci.com/smolkov/automata)
[![builds.sr.ht status](https://builds.sr.ht/~asmolkov/wqa/.build.yml.svg)](https://builds.sr.ht/~asmolkov/automata/.build.yml?)
[![open issue]][issue]
![Minimum Rust Version][min-rust-badge]

## 🎙️ Commands

`automata` is a CLI tool designed for setup and read ndir sensors data.

  - `address`: defaults name `system`. For remote acces use adress like this `tcp:host=0.0.0.0,port=6666`

  - ### 🦀⚙️ `driver`
    run driver and wath socket for connections.
    All of the arguments and flags to this command are optional:

        Build your project. This command wath link to `ndir1` directory
        with the `"type"` declared there.

  - ### 🔧 `setup`
        - `range`: defaults to `start:0.0, end: 1.0`
        - `bautrate`: default `57600`
        - `uart`:  defaults to `1` and iterate in `1..4`
      Configure sensor for user.

    ```
    wqa config --uart=2 --path=`test1`
    ```


  - ### ☁️ 🆙 `publish`

  - ### 🔬 🕵️‍♀️ `check`: run this command to confirm that your configuration is appropriately set up.


## 🔩 Configuration

`init` initialize directory.



## ⚓ Installation

1. Install `cargo`:

    Edinburgh is installed through [Cargo](https://github.com/rust-lang/cargo#compiling-from-source), a Rust package manager. Rustup, a tool for installing Rust, will also install Cargo. On Linux and macOS systems, `rustup` can be installed as follows:

    ```
    curl https://sh.rustup.rs -sSf | sh
    ```

    Additional installation methods are available [here](https://forge.rust-lang.org/other-installation-methods.html).

2. Install `automata`:

    ```
    cargo install automata
    ```

- **Troubleshooting** `libdbus-sys` errors

    On Ubuntu OS install

    ```
    $ sudo apt install libdbus-1-dev
    ```


<!-- Badges -->
[issue]: https://img.shields.io/github/issues/smolkov/automata?style=flat-square
[min-rust-badge]: https://img.shields.io/badge/rustc-1.38+-blue.svg

<!-- Server on tide [creating 🌊 web-server .deb binary with rust](https://gi.net.in/posts/creating-web-server-deb-binary-with-rust/) -->

