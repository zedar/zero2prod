use zero2prod::{adapters::httpsrv, config};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cfg = config::load()?;
    log::info!("{:?}", cfg);

    // call async routing and map std::io::Error to anyhow::Error
    httpsrv::run()?.await.map_err(anyhow::Error::from)
}
