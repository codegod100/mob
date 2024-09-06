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

// command!(foo, let x: i32, |state,x| {
//     println!("x: {}",x);
//     state.testing.clone()
// }, String);

#[tauri::command]
async fn export(state: State<'_, Mutex<AppState>>) -> Result<String, Error> {
    let state = state.lock().await;
    let e = state.keypair.export()?;
    Ok(e)
}

#[tauri::command]
async fn import(state: State<'_, Mutex<AppState>>, key: String) -> Result<String, Error> {
    let mut state = state.lock().await;
    state.keypair.import(key)?;
    Ok("ok".to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_updater::Builder::new().build())
        .setup(|app| {
            let kp = KeyPair::load();
            let kp = match kp {
                Ok(kp) => kp,
                Err(e) => {
                    eprintln!("error loading key: {}", e);
                    KeyPair::new()
                }
            };
            app.manage(Mutex::new(AppState { keypair: kp }));
            Ok(())
        })
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![sign, verify, export, import])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
