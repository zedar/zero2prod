use std::net::{SocketAddr, TcpListener};

use sqlx::{Connection, PgConnection};
use zero2prod::{adapters::httpsrv, config};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cfg = config::load()?;
    log::info!("{:?}", cfg);

    // try to connect to the database
    let db_conn = PgConnection::connect(&cfg.db.connection_string()).await?;

    // call async routing and map std::io::Error to anyhow::Error
    let listener = TcpListener::bind(SocketAddr::new(cfg.host, cfg.port))?;
    httpsrv::run(listener, db_conn)?
        .await
        .map_err(anyhow::Error::from)
}
