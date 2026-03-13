pub mod agents;
pub mod branches;
pub mod channels;
pub mod commands;
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
            commands::service::CommandServices::new_stub()
        }
    };

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(command_services)
        .invoke_handler(tauri::generate_handler![
            commands::channels::desktop_channels_list,
            commands::channels::desktop_channels_create,
            commands::channels::desktop_channels_update_status,
            commands::branches::desktop_branches_list_by_channel,
            commands::tasks::desktop_tasks_enqueue,
            commands::tasks::desktop_tasks_update_state,
            commands::tasks::desktop_tasks_list_by_channel,
            commands::workers::desktop_workers_list_by_task,
            commands::workers::desktop_workers_create,
            commands::workers::desktop_workers_update_state,
            commands::workers::desktop_workers_heartbeat,
            commands::history::desktop_history_list_by_channel,
            commands::history::desktop_history_list_by_branch,
            commands::history::desktop_history_append,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn bootstrap() -> Result<commands::service::CommandServices, core::error::MetisError> {
    core::logging::init_logging();
    let paths = core::paths::MetisPaths::resolve()?;
    let pool = tauri::async_runtime::block_on(storage::db::connect_sqlite(&paths))?;
    tauri::async_runtime::block_on(storage::migrations::run_migrations(&pool))?;
    Ok(commands::service::CommandServices::new_real(pool))
}
