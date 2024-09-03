use crypto::KeyPair;
use tauri::{async_runtime::Mutex, Manager, State};

mod crypto;

#[derive(Default, Debug)]
struct AppState {
    keypair: KeyPair,
}

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error(transparent)]
    StandardErr(#[from] std::io::Error),
    #[error(transparent)]
    AnyhowErr(#[from] anyhow::Error),
}

// impl std::fmt::Display for Error {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "oops")
//     }
// }

impl serde::Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
async fn sign(message: &str, state: State<'_, Mutex<AppState>>) -> Result<(String, String), Error> {
    println!("signing");
    let state = state.lock().await;
    let sig = state.keypair.sign(message)?;
    let pk = state.keypair.public_key()?;
    Ok((pk, sig))
}

#[tauri::command]
async fn verify(
    public_key: &str,
    message: &str,
    signature: &str,
    state: State<'_, Mutex<AppState>>,
) -> Result<bool, Error> {
    let state = state.lock().await;
    let v = state.keypair.verify(public_key, signature, message)?;
    Ok(v)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            app.manage(Mutex::new(AppState {
                keypair: KeyPair::new(),
            }));
            Ok(())
        })
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![sign, verify])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
