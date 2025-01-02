use async_trait::async_trait;

pub mod sfn;

#[async_trait]
pub trait Command {
    async fn run(self) -> color_eyre::eyre::Result<()>;
}
