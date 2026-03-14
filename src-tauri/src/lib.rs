use tauri::Manager;

pub mod agents;
pub mod app;
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
    let bootstrap = match bootstrap() {
        Ok(bootstrap) => bootstrap,
        Err(error) => {
            eprintln!("metis bootstrap failed: {error}");
            Bootstrap::new_fallback()
        }
    };

    let setup_bootstrap = bootstrap.clone();

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(bootstrap.command_services)
        .setup(move |app| {
            let sinks: Vec<std::sync::Arc<dyn app::events::DomainEventSink>> = vec![
                std::sync::Arc::new(app::events::TauriDomainEventSink::new(app.handle().clone())),
                std::sync::Arc::new(app::events::NoopDomainEventSink),
            ];
            let event_publisher = std::sync::Arc::new(app::events::FanoutDomainEventPublisher::new(sinks));
            let app_use_cases = app::use_cases::AppUseCases::new(
                setup_bootstrap.command_services.command_service_arc(),
                setup_bootstrap.runtime_sender.clone(),
                event_publisher,
            );
            app.manage(app_use_cases);
            Ok(())
        })
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

#[derive(Clone)]
struct Bootstrap {
    command_services: app::service::CommandServices,
    runtime_sender: tokio::sync::mpsc::Sender<runtime::RuntimeCommand>,
}

impl Bootstrap {
    fn new(command_services: app::service::CommandServices) -> Self {
        let runtime = runtime::RuntimeSkeleton::new(256);
        let runtime_sender = runtime.sender.clone();
        tauri::async_runtime::spawn(async move {
            runtime::run_forever(runtime.receiver).await;
        });

        Self {
            command_services,
            runtime_sender,
        }
    }

    fn new_fallback() -> Self {
        Self::new(app::service::CommandServices::new_stub())
    }
}

fn bootstrap() -> Result<Bootstrap, core::error::MetisError> {
    core::logging::init_logging();
    let paths = core::paths::MetisPaths::resolve()?;
    let pool = tauri::async_runtime::block_on(storage::db::connect_sqlite(&paths))?;
    tauri::async_runtime::block_on(storage::migrations::run_migrations(&pool))?;
    Ok(Bootstrap::new(app::service::CommandServices::new_real(pool)))
}
