pub mod adapters;
pub mod agents;
pub mod branches;
pub mod channels;
pub mod core;
pub mod history;
pub mod runtime;
pub mod storage;
pub mod tasks;
pub mod workers;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let command_services = match bootstrap() {
        Ok(command_services) => command_services,
        Err(error) => {
            eprintln!("metis bootstrap failed: {error}");
            adapters::desktop::commands::DesktopCommandServices::new_stub()
        }
    };

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(command_services)
        .invoke_handler(tauri::generate_handler![
            adapters::desktop::commands::desktop_channels_list,
            adapters::desktop::commands::desktop_channels_create,
            adapters::desktop::commands::desktop_channels_update_status,
            adapters::desktop::commands::desktop_branches_list_by_channel,
            adapters::desktop::commands::desktop_tasks_enqueue,
            adapters::desktop::commands::desktop_tasks_update_state,
            adapters::desktop::commands::desktop_tasks_list_by_channel,
            adapters::desktop::commands::desktop_workers_list_by_task,
            adapters::desktop::commands::desktop_workers_create,
            adapters::desktop::commands::desktop_workers_update_state,
            adapters::desktop::commands::desktop_workers_heartbeat,
            adapters::desktop::commands::desktop_history_list_by_channel,
            adapters::desktop::commands::desktop_history_list_by_branch,
            adapters::desktop::commands::desktop_history_append,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn bootstrap(
) -> Result<adapters::desktop::commands::DesktopCommandServices, core::error::MetisError> {
    core::logging::init_logging();
    let paths = core::paths::MetisPaths::resolve()?;
    let pool = tauri::async_runtime::block_on(storage::db::connect_sqlite(&paths))?;
    tauri::async_runtime::block_on(storage::migrations::run_migrations(&pool))?;
    Ok(adapters::desktop::commands::DesktopCommandServices::new_real(pool))
}
