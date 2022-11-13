<div align="center">
    <img src="https://github.com/emberry-org/emberry-rework/blob/main/.github/assets/banner.png"> 
</div>
<div align="right"><sup>Powered by <a href="https://github.com/tauri-apps/tauri">Tauri</a> & <a href="https://github.com/sveltejs/kit">SvelteKit</a> ⠿</sup></div>

<h3 align="center">
    A new way to Chat with Friends
</h3>
    
<br>
    
Emberry is a peer to peer chat application with the mission<br>
To create a **secure** and **efficient** chat platform for everyone.<br>
<sub>⠿ <i>Emberry is still very early in its development, if <b>you</b> want to contribute <a href="https://mxcop.github.io/portfolio">contact me</a> :D</i></sub>

<br>

<div align="left">
    <img width="80%" src="https://github.com/emberry-org/emberry-rework/blob/main/.github/assets/dev-banner.png"> 
    <img align="right" width="8%" src="https://github.com/emberry-org/emberry-rework/blob/main/.github/assets/logo.png"> 
</div>

<h3>Frontend</h3>
Install Javascript dependencies :
<div align="right"><sup>We use Yarn as our package manager ⠿</sup></div>

```
$ yarn
```

<br>

Start the development environment :
```
$ yarn tauri dev
```

<br>

<h3>Backend</h3>

For the backend setup add a ``.env`` file to the ``src-tauri`` directory :

```js
/src-tauri/.env

SERVER_ADDRESS=<server_ip>:<udp_port>
CONTROL_ADDRESS=<server_ip>:<ctrl_chnl_port>
SERVER_DOMAIN=<certificate domain name>
CERT=<server X509Certificate>
```

<br>

Then add a ``build/`` directory on the same level as ``src/`` and ``src-tauri/`` as shown below :
```graphql
./emberry/* 
  ├─ build/      - # Vite build output
  ├─ src/        - # Frontend codebase
  ├─ src-tauri/  - # Backend codebase
  ├─ static/     - # Production assets (included within the build)
  └─ README.md
```

<br>

<h3>Dependencies</h3>

To build Emberry on arch based systems the following packages need to be installed :

```
$ pacman -S webkit2gtk
```

<br>

<h3>Running</h3>

To run Emberry in developer mode use the following command :

```
$ yarn tauri dev
```
