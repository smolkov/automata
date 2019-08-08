#   `wqa`

 **📦  water quality analyzers rewrite in [🦀 **Rust**](https://github.com/lar-rs/wqa)**

* Design-UI package : :iphone:
🚧 _Work In Progress_ 🚧

* **TODO: Welche Aufgaben hat es?**


 * �

[![travis build Status](https://travis-ci.com/lar-rs/edinburgh.svg?branch=master)](https://travis-ci.com/lar-rs/wqa)
[![builds.sr.ht status](https://builds.sr.ht/~asmolkov/wqa/.build.yml.svg)](https://builds.sr.ht/~asmolkov/wqa/.build.yml?)
[![open issue]][issue]
![Minimum Rust Version][min-rust-badge]

## 🎙️ Commands

`wqa` is a CLI tool designed for setup and read ndir sensors data.

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

2. Install `wqa`:

    ```
    cargo install wqa
    ```

- **Troubleshooting** `libdbus-sys` errors

    On Ubuntu OS install

    ```
    $ sudo apt install libdbus-1-dev
    ```


<!-- Badges -->
[irc]:          https://webirc.hackint.org/#irc://irc.hackint.org/#lar
[issue]: https://img.shields.io/github/issues/lar-rs/wqa?style=flat-square
[min-rust-badge]: https://img.shields.io/badge/rustc-1.38+-blue.svg

<!-- Server on tide [creating 🌊 web-server .deb binary with rust](https://gi.net.in/posts/creating-web-server-deb-binary-with-rust/) -->

## `Water quality monitoring station`

 `umweltanalytik` ist ein Teilbereich der chemischen Analytik und beschäftigt sich mit der qualitativen und quantitativen Untersuchung von Stoffen in der Umwelt. Umweltkompartimente Luft (einschließlich Innenraumluft), Boden und **Wasser** und können sowohl einzelne Stoffe als auch Summenparameter umfassen.

[Wiki](https://de.wikipedia.org/wiki/Umweltanalytik)

## Summenparameter in der Wasseranalytik

- Biochemischer Sauerstoffbedarf (BSB)
- Permanganat-Index (PI)
- Chemischer Sauerstoffbedarf (CSB, gelegentlich noch CSV - Chemischer Sauerstoffverbrauch, engl. Chemical Oxygen Demand, COD)
- Gesamter Organischer Kohlenstoff (Total Organic Carbon, TOC)
- Gesamter gelöster organischer Kohlenstoff (Dissolved Organic Carbon, DOC)
- Leitfähigkeit

***Gruppenparameter?***

- Adsorbierbare organisch gebundene Halogene, AOX (X steht für Halogene)
- Extrahierbare organisch gebundene Halogene, EOX
- Ausblasbare (flüchtige) organisch gebundene Halogene (POX, von „purgable“)

Anwendung in `Water Quality Monitoring Station` sollte die Umstieg von C auf Rust erleichtern.



## Usage
`wqa` requires `rustc` 1.28.0 or later. Add this to your `Cargo.toml`:

```toml
[dependencies]
wqa = {path="../wqa"}
```

## Compiling

Requires Rust nightly. To compile using [`rustup`](https://rustup.rs/):

```ShellSession
$ rustup toolchain install nightly
$ rustup default nightly
$ cargo build
```

Be sure to switch back to `stable` with `rustup default stable` if that's your preferred toolchain.


To cross-compile for the Raspberry Pi you will need an
`gcc-multilib-i686-linux-gnu` GCC toolchain and Rust component installed. Add the Rust target
with `rustup target add i686-unknown-linux-gnu`. Then you can
cross-compile with `cargo`:

```ShellSession
    cargo build --release --target i686-unknown-linux-gnu
```

<!-- [irc-img]:      https://img.shields.io/badge/hackint-%23lar-blue.svg -->
<!-- [ui]: https://user-images.githubusercontent.com/383250/59148363-53188c80-8a08-11e9-9b29-9cac56809ee2.png "Automaat UI Example" -->

