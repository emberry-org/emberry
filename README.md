# Emberry

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
