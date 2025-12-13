mod chrome;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    chrome::launch_manual_devtools_session().await
}
