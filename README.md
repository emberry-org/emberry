<h1 align="center">
    <div align="center">
        <img width=140 src="https://github.com/emberry-org/emberry-rework/blob/main/.github/assets/logo.png"> 
    </div>
</h1>


<div align="center">
    <b>Emberry</b> - Chatting directly with friends <b>in privacy!</b><br>
</div>
    
<br>

<h1><samp>Development</samp></h1>

<br>

<h3><samp>Frontend</samp></h3>

For the frontend setup install the node packages using the following command :

```
$ yarn
```

<br>

<h3><samp>Backend</samp></h3>

For the backend setup add a ``.env`` file to the ``src-tauri`` directory :

```js
/src-tauri/.env

SERVER_ADDRESS=<server_ip>:<udp_port>
SERVER_DOMAIN=<certificate domain name>
CONTROL_ADDRESS=<server_ip>:<ctrl_chnl_port>
PUBLIC_KEY=<32 byte string>
CERT=<X509Certificate>
```

<br>

Then add a ``build/`` directory on the same level as ``src/`` and ``src-tauri/`` as shown below :
```
.
├─ build/
├─ src/
├─ src-tauri/
├─ public/
└─ README.md
```

<br>

<h3><samp>Dependencies</samp></h3>

To build emberry on arch based systems the following packages need to be installed:

```
$ pacman -S webkit2gtk
```

<br>

<h3><samp>Running</samp></h3>

To run Emberry in developer mode use the following command :

```
$ yarn tauri dev
```
