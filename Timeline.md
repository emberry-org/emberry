# Timeline

<br>

### Markers

:grapes: Emberry <br>
:seedling: Rhizome

<br>

### Direct Messaging

#### Phase A
- [x] :grapes: :seedling: TLS control channel for rendezvous server
- [x] :grapes: :seedling: Try open room at "Pubkey1 + Pubkey2" (both parties have to do that and know the room ID because its unique)
- [ ] :grapes: Messages types (message, set_name, etc)
- [ ] :grapes: Simultaneous rooms (multiple rooms open at once) (https://tauri.studio/v1/guides/features/command/#async-commands)
- [x] :grapes: Unencrypted ephemeral messages
- [ ] :grapes: Exchange usernames with comminication partner <br>
#### Phase B
- [ ] :seedling: Interactive room requests
  - [ ] :seedling: Relay room open request to peer over control channel
  - [ ] :grapes: Auto accept room open request if peer is in address book
- [ ] :grapes: Message received and order ensurance (TCP-like promises)
- [ ] :grapes: Unencrypted persistent messages <br>
- [ ] :grapes: Public key storage (address book) (https://github.com/tauri-apps/tauri-plugin-stronghold)
#### Phase C
- [ ] :seedling: Live user info on server
  - [ ] :seedling: Allow user to set a Username and status (username from client cannot contain '#')
  - [ ] :seedling: status auto change to offline when disconnected from TLS control port
  - [ ] :grapes: :seedling: Query user info by (username, pubkey) -> (pubkey, status) <br>
#### Phase D
- [ ] :grapes: Encrypted messages (OTP or single AES Key)
- [ ] :grapes: Encrypted authenticated messages (signature) (signature on username)

<br>

### Ideas

- [ ] Co-op networking local API
- [ ] Controller / Keyboard / Mouse emulator for screensharing
