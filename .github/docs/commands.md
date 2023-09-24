# Tauri Commands

### Get User Info <sub>`get_usr_info`</sub>
**@Inputs**:
```bash
bs58cert: String    # User certificate in base 58
```
**@Returns**:
```bash
{
    # Username of the user
    username: String,
    # Our relation to the user
    relation: UserRelation {
        Undefined = 0,
        Known     = 1,
        Friend    = 2,
        Stranger  = 3,
        Local     = 255
    },
}
```

<br>

### Get Users <sub>`get_usrs`</sub>
**@Inputs**
```bash
limit: i64,     # Maximum number of users to fetch
offset: usize,  # Offset for loading pages of users
```
**@Returns**
```bash
{
    identifier: UserIdentifier {
        # User certificate in base 58
        bs58: String
    },
    info: {
        # Username of the user
        username: String,
        # Our relation to the user
        relation: UserRelation {
            Undefined = 0,
            Known     = 1,
            Friend    = 2,
            Stranger  = 3,
            Local     = 255
        },
    }
} | ERROR
```

#[tauri::command]
pub fn get_local<'a>() -> Option<IdentifiedUserInfo<'a>> {
    let lock = config::IDI.read().unwrap();
    lock.clone()
}

#[tauri::command]
pub fn update_username(window: Window, name: String) {
    let frontend_event = |info: &IdentifiedUserInfo| {
        let event = format!("usr_name_{}", info.identifier.bs58);
        if let Err(err) = window.emit(&event, &info.info.username) {
            log::error!("Failed to emit event: '{}'", err);
        }
    };
    let mut lock = config::IDI.write().unwrap();
    let option = lock.as_mut();
    if let Some(mut id_info) = option {
        if name != id_info.info.username {
            id_info.info.username = name;
            match try_exec(upsert, (id_info, frontend_event)) {
                Ok(()) => (),
                Err(err) => log::warn!("Could not update local username: '{}'", err),
            }
        }
    }
}

#[tauri::command]
pub fn generate_user_certificate() {
    cert_gen::generate_cert(&config::PEM.filepath).unwrap()
}

#[tauri::command]
pub fn chat_exists(state: tauri::State<'_, Networking>, id: RoomId) -> bool {
    // Check if the store contains the key for this chat.
    match state.chats.lock() {
        Ok(chats) => chats.contains_key(&id),
        Err(_) => false,
    }
}

#[tauri::command(async)]
pub async fn connect(
    window: tauri::Window,
    app_handle: tauri::AppHandle,
    net: tauri::State<'_, Networking>,
    rc: tauri::State<'_, RhizomeConnection>,
) -> tauri::Result<()>

pub async fn request_room(
    window: Window,
    bs58cert: String,
    net: tauri::State<'_, Networking>,
    rc: tauri::State<'_, RhizomeConnection>,
) -> tauri::Result<()>

pub async fn accept_room(
    bs58cert: String,
    accepted: bool,
    net: tauri::State<'_, Networking>,
    rc: tauri::State<'_, RhizomeConnection>,
) -> Result<()>

usr_name_<peer_id>
