[![pipeline status](https://gitlab.com/asmolkov/automata/badges/master/pipeline.svg)](https://gitlab.com/brndnmtthws-oss/labhub/commits/master) ![Current Crates.io Version](https://img.shields.io/crates/v/automata.svg)](https://crates.io/crates/automata)

# 🤖 `automata`

A automation freak written in Rust


🚧 _Work In Progress_ 🚧


[ui]: https://user-images.githubusercontent.com/383250/59148363-53188c80-8a08-11e9-9b29-9cac56809ee2.png "Automaat UI Example"
## Features

- **TODO**

### Commands

Commands can be executed by commenting on a PR with your CI user's login.

- **`@automata info`**: retry a pipeline that has failed

## The Problem


## ✨ The Solution


## 🏃‍♀️ In Action

**Using automata?😀**


## Compiling

automata requires Rust nightly. To compile using [`rustup`](https://rustup.rs/):

```ShellSession
$ rustup toolchain install nightly
$ rustup default nightly
$ cargo build
```

Be sure to switch back to `stable` with `rustup default stable` if that's your preferred toolchain.

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
$ ssh-keygen -f labhub-key.ecdsa -t ecdsa -b 521
```

Keep `automata-key.ecdsa` safe, and upload `automata-key.ecdsa.pub` to both GitHub and GitLab for the CI user.

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

### Deploy to Kubernetes with Helm

There's a Helm chart included in this repo, which is the preferred method of deployment. To use you, you must first create the SSH key secrets with kubectl. Assuming your SSH private key is `labhub-key.ecdsa`:

```ShellSession
$ kubectl create secret generic labhub-ssh-keys --from-file=github=labhub-key.ecdsa --from-file=gitlab=labhub-key.ecdsa
```

You may use separate keys for GitHub and GitLab if you choose, respectively.

Once you have the secrets, install the helm chart from [helm/labhub/](helm/labhub/):

```ShellSession
$ cd helm/labhub/
$ cp values.yaml myvalues.yaml
### Edit myvalues.yaml to your liking ###
$ helm upgrade --install labhub . -f myvalues.yaml
```

### Not implemented:

- No periodic reconciling of GitLab branches with open PRs: if a webhook is missed for any reason, the GitLab pipeline may not correctly reflect the PR state
