# 📦 `wqa`

🚧 _Work In Progress_ 🚧

<h1 align="center">Water quality analyzer</h1>
<div align="center">
 <strong>
   Empowering everyone to rewrite LAR pwa software in rust
 </strong>
</div>

<div align="center">
  <!-- Crates version -->
  <a href="https://crates.io/crates/wqa">
    <img src="https://img.shields.io/crates/v/wqa.svg?style=flat-square"
    alt="Crates.io version" />
  </a>
  <!-- Build Status -->
  <a href="https://travis-ci.org/lar-rs/wqa">
    <img src="https://travis-ci.com/lar-rs/wqa.svg?branch=master"
      alt="Build Status" />
  </a>
  <!-- Downloads -->
  <a href="https://crates.io/crates/wqa">
    <img src="https://img.shields.io/crates/d/wqa.svg?style=flat-square"
      alt="Download" />
  </a>
  <!-- docs.rs docs -->
  <a href="https://docs.rs.lar.de/wqa">
    <img src="https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square"
      alt="LAR docs.rs docs" />
  </a>
</div>

<div align="center">
  <h3>
    <a href="https://docs.rs.lar.de/wqa">
      API Docs
    </a>
    <span> | </span>
    <a href="https://github.com/rustasync/wqa/blob/master/.github/CONTRIBUTING.md">
      Contributing
    </a>
    <span> | </span>
    <a href="https://lar.zulipchat.com/">
      Chat
    </a>
  </h3>
</div>

<div align="center">
  <sub>Built with 🌊 by <a href="https://github.com/lar-rs">The LAR Ecosystem WG</a>
</div>


## About

LAR water quality analyzers rewrite in [🦀 **Rust**](https://github.com/lar-rs/wqa)

[![travis build Status](https://travis-ci.com/lar-rs/wqa.svg?branch=master)](https://travis-ci.com/lar-rs/wqa)
[![builds.sr.ht status](https://builds.sr.ht/~asmolkov/wqa/.build.yml.svg)](https://builds.sr.ht/~asmolkov/wqa/.build.yml?)
[![open issue][issue]]
![Minimum Rust Version][min-rust-badge]

## 🎙️ Commands

`wqa` is a CLI tool designed for folks who are interested in using Cloudflare workers.

<!-- Server on tide [creating 🌊 web-server .deb binary with rust](https://gi.net.in/posts/creating-web-server-deb-binary-with-rust/) -->

## `Water quality monitoring station`

@font:`PragmataPro`

*Move it to gui readme*
[Hello Rust](https://github.com/hello-rust)
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

[irc-img]:      https://img.shields.io/badge/hackint-%23lar-blue.svg
[irc]:          https://webirc.hackint.org/#irc://irc.hackint.org/#lar

[ui]: https://user-images.githubusercontent.com/383250/59148363-53188c80-8a08-11e9-9b29-9cac56809ee2.png "Automaat UI Example"
[issue]: https://img.shields.io/github/issues/lar-rs/wqa?style=flat-square

<!-- Badges -->
[min-rust-badge]: https://img.shields.io/badge/rustc-1.38+-blue.svg

