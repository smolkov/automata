#  `wqa`

LAR water quality analyzers  [🦀 **Rust**](https://github.com/wasmerio/wasmer-rust-example)

![pipeline status](https://travis-ci.org/lar-rs/wqa.svg?branch=master)
[![builds.sr.ht status](https://builds.sr.ht/~asmolkov/wqa/.build.yml.svg)](https://builds.sr.ht/~asmolkov/wqa/.build.yml?)


A automation bot written in Rust.
Server on tide [creating 🌊 web-server .deb binary with rust](https://gill.net.in/posts/creating-web-server-deb-binary-with-rust/)

🚧 _Work In Progress_ 🚧

![Minimum Rust Version][min-rust-badge]

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

`target/i686-unknown-linux-gnu/release/wqm-uv`


[ui]: https://user-images.githubusercontent.com/383250/59148363-53188c80-8a08-11e9-9b29-9cac56809ee2.png "Automaat UI Example"

## Features

-  **TODO:**

### :bookmark_tabs: Commands
Commands can be executed by commenting on a PR with your CI user's login.

View the options with `./wqm-uv -h`. By default it will try to bind the
webserver to port 80. You can give a regular user the permission to do this
with:

- **`@wqm-uv info`**:  retry a pipeline that has failed

##  The Problem


## ✨ The Solution


## 🏃‍♀️ In Action

## Endpoints

- `GET`
  - `/`  :  Print `Hello World`
  - `/user` : Print all of users
  - `/user/:id` : Print `:id` user
  - `/api/stream` : Print all streams
  - `/api/rule` : Print all rules
- `POST`
  -  `/stream` : Create new user ( With `{"name": "user_name", "height": INTEGER}` body params )
- `PATCH`
   - `/user/:id` : Update user ( With `{"name": "user_name", "height": INTEGER}` body params)
- `DELETE`
  - `/user/:id` : Delete user
**Using wqm-uv?😀**



## 🎛 Configuration

Automata is configured using [`automata.toml`](automata.toml). For details, see [src/config.rs](src/config.rs). You can specify the path to `automata.toml` by setting the `AUTOMATA_TOML` environment variable.

## 🚀 Deployment


### Setup Webhooks

You'll need to set up webhooks for any repo you wish to enable LabHub for. Currently, only GitHub webhooks are required. To get started, go to `github.com/<org>/<repo>/settings/hooks` and add a new webhook.

Configure the webhook to send PR and push events.

- Set the payload URL path to `/github/events`, which is the path LabHub is expecting for GitHub events.
- Create a secrete (ex: `cat /dev/urandom | LC_CTYPE=C tr -dc 'a-zA-Z0-9' | fold -w 32 | head -n 1`) and set the same value in the webhook config as in LabHub.
- Make sure the payload type is `application/json`.
- [Here's how your webhook should look](docs/github-webhook-config.png)

### Create SSH keys

You'll need a CI user with SSH keys for both GitHub and GitLab. Create an account on both sites (if you don't already have a CI user), and create an SSH key for LabHub:

```ShellSession
$ ssh-keygen -f wqa-key.ecdsa -t ecdsa -b 521
```

Keep `wqa-key.ecdsa` safe, and upload `automata-key.ecdsa.pub` to both GitHub and GitLab for the CI user.

### Create Personal Access Tokens

Create personal access tokens for your CI user on both GitHub, and GitLab. Supply these tokens by setting the `api_token` parameter in `LabHub.toml` for both GitHub and GitLab.

#### Personal Access Token for GitHub

- Go to https://github.com/settings/tokens
- Click "Generate new token"
- Give the token a name, and [enable the `repo` scope, like this](docs/github-personal-access-token.png).
- Save that token to your `LabHub.toml`

#### Personal Access Token for GitLab

- Go to https://gitlab.com/profile/personal_access_tokens
- Give the token a name, and [enable the `api` scope, like this](docs/gitlab-personal-access-token.png).
- Save that token to your `LabHub.toml`

### Access Token for git.sr.ht
- Go to

### Not implemented:




<!-- Badges -->
[min-rust-badge]: https://img.shields.io/badge/rustc-1.38+-blue.svg

