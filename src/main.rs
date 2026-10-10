//! RimePanel - a lightweight Minecraft server web panel.
mod api;
mod auth;
mod backup;
mod clientmods;
mod curseforge;
mod db;
mod files;
mod install;
mod instance;
mod java;
mod library;
mod modrinth;
mod net;
mod props;
mod slp;
mod state;
mod tasks;
mod util;
mod versions;
mod worlds;

use std::{net::SocketAddr, path::PathBuf, sync::Arc};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "rimepanel=info,tower_http=warn".into()),
        )
        .init();

    let mut bind: SocketAddr = "0.0.0.0:8080".parse().unwrap();
    let mut data = PathBuf::from("./data");
    let mut args = std::env::args().skip(1);
    while let Some(a) = args.next() {
        match a.as_str() {
            "--bind" | "-b" => bind = args.next().expect("--bind <addr>").parse()?,
            "--data" | "-d" => data = PathBuf::from(args.next().expect("--data <dir>")),
            "--help" | "-h" => {
                println!("rimepanel [--bind 0.0.0.0:8080] [--data ./data]");
                return Ok(());
            }
            other => anyhow::bail!("unknown argument: {other}"),
        }
    }
    if let Ok(v) = std::env::var("RIMEPANEL_BIND") {
        bind = v.parse()?;
    }
    if let Ok(v) = std::env::var("RIMEPANEL_DATA") {
        data = PathBuf::from(v);
    }

    let app = state::AppState::init(data).await?;
    let app = Arc::new(app);
    instance::start_background(app.clone());
    backup::start_scheduler(app.clone());

    let router = api::router(app.clone());
    let listener = tokio::net::TcpListener::bind(bind).await?;
    tracing::info!("RimePanel listening on http://{bind}");

    let shutdown_app = app.clone();
    axum::serve(listener, router.into_make_service_with_connect_info::<SocketAddr>())
        .with_graceful_shutdown(async move {
            shutdown_signal().await;
            tracing::info!("shutting down, stopping running servers...");
            shutdown_app.instances.stop_all().await;
        })
        .await?;
    Ok(())
}

async fn shutdown_signal() {
    let ctrl_c = async {
        let _ = tokio::signal::ctrl_c().await;
    };
    #[cfg(unix)]
    let term = async {
        if let Ok(mut s) =
            tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
        {
            s.recv().await;
        }
    };
    #[cfg(not(unix))]
    let term = std::future::pending::<()>();
    tokio::select! { _ = ctrl_c => {}, _ = term => {} }
}
