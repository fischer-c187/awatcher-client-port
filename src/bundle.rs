mod menu;
mod modules;
mod server;

pub use menu::Tray;
use std::path::PathBuf;
use tokio::sync::mpsc::UnboundedSender;

pub async fn run(
    host: String,
    port: u16,
    config_file: PathBuf,
    no_tray: bool,
    run_server: bool,
    shutdown_sender: UnboundedSender<()>,
) {
    let manager = modules::Manager::new(
        &std::env::var("PATH").unwrap_or_default(),
        config_file.parent().unwrap(),
    );

    if !no_tray {
        let tray = Tray::new(host.clone(), port, config_file, shutdown_sender, manager);
        let service = ksni::TrayService::new(tray);
        service.spawn();
    }

    if run_server {
        server::run(host.clone(), port).await;
    } else {
        info!("Bundled server disabled by config; not starting aw-server.");
        std::future::pending::<()>().await;
    }
}
