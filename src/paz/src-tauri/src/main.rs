#![cfg_attr(all(not(debug_assertions), target_os = "windows"), windows_subsystem = "windows")]

pub(crate) 
use pazcore::{Core, ClientCommand, ClientQuery, CoreResponse};
use tauri::api::path;
use tauri::{
  SystemTray, 
  CustomMenuItem, 
  SystemTrayMenu, 
  SystemTrayMenuItem, 
  AppHandle, 
  SystemTrayEvent, 
  RunEvent, 
  WindowBuilder, 
  Manager, 
  WindowUrl
};


#[tauri::command]
fn client_query(core: tauri::State<'_, Core>, data: ClientQuery) -> Result<CoreResponse, String> {
  match core.exec_query(data) {
    Ok(response) => Ok(response),
    Err(err) => {
      println!("Error: Query: {:?}", err);
      Err(err.to_string())
    }
  }
}

#[tauri::command]
fn client_command(core: tauri::State<'_, Core>, data: ClientCommand) -> Result<CoreResponse, String> {
  match core.exec_command(data) {
    Ok(response) => Ok(response),
    Err(err) => {
      println!("Error: Command: {:?}", err);
      Err(err.to_string())
    }
  }
}




fn main() {

  // instantiate core
  let data_dir = path::data_dir().unwrap_or(std::path::PathBuf::from("./"));
  let core = Core::new(data_dir);

  // init connections/network resources
  core.initialize();

  // core.start in a background thread

  // build sys tray
  let mut status = CustomMenuItem::new("status", "Status");
  status.enabled = false;
  let open = CustomMenuItem::new("open", "Open");
  let exit = CustomMenuItem::new("exit", "Exit");

  let menu = SystemTrayMenu::new()
    .add_item(status)
    .add_native_item(SystemTrayMenuItem::Separator)
    .add_item(open)
    .add_item(exit);

  let tray = SystemTray::new()
    .with_menu(menu);



  // build app
  let app = tauri::Builder::default()
    .manage(core)
    .invoke_handler(tauri::generate_handler![client_query, client_command])
    .system_tray(tray)
    .on_system_tray_event(system_tray_event_handler)
    .build(tauri::generate_context!())
    .expect("error while running tauri application");

  // run
  app.run(run_event_handler);
}


pub fn run_event_handler(_: &AppHandle, event: RunEvent) {
  match event {
    tauri::RunEvent::ExitRequested { api, .. } => {
      api.prevent_exit()
    }
    _ => {}
  }
}

pub fn system_tray_event_handler(handler: &AppHandle, event: SystemTrayEvent) {
  match event {
    SystemTrayEvent::DoubleClick { position: _, size: _, .. } => println!("Info: SysTray: DoubleClick"),
    SystemTrayEvent::RightClick { position: _, size: _, .. } => println!("Info: SysTray: RightClick"),
    SystemTrayEvent::LeftClick { position: _, size: _, .. } => println!("Info: SysTray: LeftClick"),
    SystemTrayEvent::MenuItemClick { id, .. } => {
      match id.as_str() {
          "exit" => handler.exit(0),
          "open" => open_app_event_handler(handler),
          _ => println!("Info: SysTray: {0} Clicked", id)
      }
    }
    _ => todo!(),
  }
}

pub fn open_app_event_handler(handler: &AppHandle) {

  if let Some(w) = handler.get_window("main") {
    println!("Warn: SysTray: The window is already open");
    w.set_focus().unwrap();
    return;
  }


  if let Some(config) = handler
    .config()
    .clone()
    .tauri
    .windows.first() {

      _ = WindowBuilder::new(handler, "main", WindowUrl::App("index.html".into()))
      .title(config.title.clone())
      .inner_size(config.width, config.height)
      .resizable(config.resizable)
      .fullscreen(config.fullscreen)
      .build()
      .unwrap();
      return;
    }

    println!("Error: SysTray: Cannot find window config")
}

