# Emberry Chat
A **P2P** chat application which **empowers friends** to communicate **efficiently** and **safely**.

## Packages
```bash
# frontend framework
solid.js    18.1 kB | gzip: 6.4 kB

# application framework
tauri           n/a | n/a
```

<br>

## Plugins
[`vite-plugin-solid-svg`](https://github.com/jfgodoy/vite-plugin-solid-svg) — Static svg loader.<br>

<br>

## CLI Commands
```bash
# install pnpm
$ npm install -g pnpm

# install dependencies
$ pnpm install

# run app in dev mode (for testing)
$ pnpm tauri dev

# build frontend (useful for viewing file sizes)
$ pnpm run build

# build app
$ pnpm tauri build

# enable logging (windows)
$ $Env:RUST_LOG="emberry,smoke,emberry_rs::network::p2p_tunl::p2p_loop=warn"
```

<br>

## Importing Icons
```tsx
import Icon from '@ico/icon.svg?component-solid';

<>
    <Icon width="24px" />
</>
```

<br>

<sub>Build with [Solid JS](https://solidjs.com), [Tauri](https://tauri.app), & [Rust-lang](https://www.rust-lang.org)</sub>
<h2></h2>
<div align="right"><sub>© 2023 Devensiv & Max, All rights reserved — <a href="./license.md">GNU GPLv3</a>.</sub></div>
