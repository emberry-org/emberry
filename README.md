<h1 align="center">
    <div align="center">
        <img width=140 src="https://github.com/emberry-org/emberry-rework/blob/main/.github/assets/logo.png"> 
    </div>
</h1>


<div align="center">
    <b>Emberry</b> - Chatting directly with friends <b>in privacy!</b><br>
</div>
    
<br>

## Development

<br>

### Frontend

For the frontend setup install the node packages using the following command :

```
$ npm i
```

<br>

### Backend

For the backend setup add a ``.env`` file to the ``src-tauri`` directory :

```js
/src-tauri/.env

SERVER_ADDRESS= ...
```

<br>

Then add a ``dist/`` directory on the same level as ``src/`` and ``src-tauri/`` as shown below :
```
.
├─ dist/
├─ src/
├─ src-tauri/
├─ public/
└─ README.md
```

<br>

### Running

To run Emberry in developer mode use the following command :

```
$ npm run tauri dev
```
