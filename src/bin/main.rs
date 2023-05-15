use std::net::{SocketAddr, TcpListener};

use sqlx::PgPool;
use zero2prod::{adapters::httpsrv, config};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cfg = config::load()?;
    log::info!("Configuration {:?}", cfg);

    // build database connection pool
    let db_conn_pool = PgPool::connect(&cfg.db.connection_string()).await?;

    // call async routing and map std::io::Error to anyhow::Error
    let listener = TcpListener::bind(SocketAddr::new(cfg.host, cfg.port))?;
    httpsrv::run(listener, db_conn_pool)?
        .await
        .map_err(anyhow::Error::from)
}
