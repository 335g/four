use cargo_four::run;

#[tokio::main]
async fn main() -> color_eyre::eyre::Result<()> {
    color_eyre::install()?;
    run().await?;

    Ok(())
}
