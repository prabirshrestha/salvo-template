use crate::{AppResult, app::App, controllers};

use salvo::{catcher::Catcher, prelude::*};
use std::net::SocketAddr;
use tokio_util::sync::CancellationToken;
use tracing::info;

pub async fn run_web_server(app: App, cancel_token: CancellationToken) -> AppResult<()> {
    info!("Starting web server...");
    let mut listenfd = listenfd::ListenFd::from_env();
    let (addr, listener) = if let Some(listener) = listenfd.take_tcp_listener(0)? {
        listener.set_nonblocking(true)?;
        (
            listener.local_addr()?,
            tokio::net::TcpListener::from_std(listener).unwrap(),
        )
    } else {
        let addr: SocketAddr = format!("{}:{}", &app.app_config.host, &app.app_config.port)
            .parse()
            .unwrap();
        (addr, tokio::net::TcpListener::bind(addr).await.unwrap())
    };

    tracing::info!("Listening on {}", addr);

    let acceptor = salvo::conn::tcp::TcpAcceptor::try_from(listener).unwrap();

    let ct = cancel_token.clone();
    let webserver = Server::new(acceptor);
    let webserver_handle = webserver.handle();

    tokio::spawn(async move {
        ct.cancelled().await;
        webserver_handle.stop_graceful(Some(tokio::time::Duration::from_secs(15)));
    });

    webserver
        .serve(
            Service::new(
                Router::new()
                    .hoop(salvo::affix_state::inject(app.clone()))
                    .hoop(Logger::default())
                    .push(controllers::router()),
            )
            .catcher(Catcher::default().hoop(controllers::errors::internal_server_error)),
        )
        .await;

    Ok(())
}
