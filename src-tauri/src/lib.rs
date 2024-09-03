use crypto::KeyPair;
use tauri::{async_runtime::Mutex, Manager, State};

mod crypto;

#[derive(Default, Debug)]
struct AppState {
    keypair: KeyPair,
}

#[derive(Debug, thiserror::Error)]
enum Error{
    #[error(transparent)]
    StandardErr(#[from] std::io::Error),
    AnyhowErr(#[from] anyhow::Error)
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result{
        write!(f, "oops {}",self)
    }

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
async fn greet(name: &str, state: State<'_, Mutex<AppState>>) -> Result<String,Error> {
    let mut state = state.lock().await;
    let sig = state.keypair.sign(name)?;                                                                                                                                                                                                                                                    
    Ok(format!("Hello, {}! You've been meated from Rust! {:#?}", name, sig))
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            app.manage(Mutex::new(AppState{
                keypair: KeyPair::new()
            }));
            Ok(())
        })
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
