use crypto::KeyPair;
use tauri::{async_runtime::Mutex, Manager, State};
mod crypto;
use std::path::PathBuf;
use tauri::Wry;
use tauri_plugin_store::{with_store, StoreCollection};

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

#[tauri::command]
async fn reset(state: State<'_, Mutex<AppState>>) -> Result<String, Error> {
    let mut state = state.lock().await;
    state.keypair.reset();
    Ok("ok".to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let builder = tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_devtools::init());
    #[cfg(not(any(target_os = "android", target_os = "ios")))]
    let builder = builder.plugin(tauri_plugin_updater::Builder::new().build());
    builder
        .setup(|app| {
            let stores = app
                .handle()
                .try_state::<StoreCollection<Wry>>()
                .ok_or("Store not found")?;
            let path = PathBuf::from("store.bin");
            let mut key_str: Option<String> = None;
            with_store(app.handle().clone(), stores, path, |store| {
                let key = store.get("key");
                if let Some(key) = key {
                    if let Some(key) = key.as_str() {
                        println!("Loading with key discovered in store");
                        key_str = Some(key.to_string());
                    }
                }
                Ok(())
            })?;
            let kp = match key_str {
                Some(key_str) => KeyPair::new_with_key(&key_str)?,
                None => KeyPair::new(),
            };

            app.manage(Mutex::new(AppState { keypair: kp }));
            Ok(())
        })
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            sign, verify, export, import, reset
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
