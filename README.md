# Emberry Chat
A **P2P** chat application which **empowers friends** to communicate **efficiently** and **safely**.

<br>

# Building from source
1. Make sure you are in a suitable [dev environment](#required-build-tools)
2. install **js** dependencies, run **`pnpm i --frozen-lockfile`**
3. clone the repository and navigate to the project root
    3.1. **`git clone https://github.com/emberry-org/emberry.git`**
    3.2. **`cd emberry`**

Now you may either run the project in debug mode:
```bash
pnpm tauri dev
```
or you can create a build like this:
```bash
pnpm tauri build
```
The final build can be found at `src-tauri/target/release/emberry` relative to the root of the cloned repository.

## Using cargo tauri cli
If you are using the nix environment you can also run **`cargo tauri dev`** and **`cargo tauri build`** instead

If you do not use the nix environment but still like to use the `cargo` version of these commands you may install
```bash
cargo install tauri-cli
```

## Logging
The log level is set at runtime through the `RUST_LOG` environment variable.

To enable all logging for the local code set `RUST_LOG=emberry_rs`

If you are using the nix development environment logging on trace level is enabled by default.

### Powershell
``` powershell
$Env:RUST_LOG="emberry_rs"
```
### Bash
``` bash
export RUST_LOG=emberry_rs
```
### cmd
``` cmd
SET RUST_LOG=emberry_rs
```

# Required build tools

## Using Nix
The easiest way to get a reliable build environment is by using [nix](https://nixos.org/download.html)

This project uses nix flakes.

### Enable nix flakes
To enable nix flakes either `~/.config/nix/nix.conf` or `/etc/nix/nix.conf` need to contain:
```
experimental-features = nix-command flakes
```
If the Nix installation is in multi-user mode, don’t forget to restart the nix-daemon.

### Use Nix Shell
If your Nix is set up properly **`nix develop`** started inside the project dir should just work.

Note that the first time you run this all the necessary tooling needs to be downloaded which might take some time.

## Without Nix
To build emberry you will need to install:
 - [pnpm](https://pnpm.io/installation) v8.0.0 (or newer)
 - [rust-lang](https://www.rust-lang.org/tools/install) v1.56.0 (or newer)



<sub>Build with [Solid JS](https://solidjs.com), [Tauri](https://tauri.app), & [Rust-lang](https://www.rust-lang.org)</sub>
<h2></h2>
<div align="right"><sub>© 2023 Devensiv & Max, All rights reserved — <a href="./license.md">GNU GPLv3</a>.</sub></div>

