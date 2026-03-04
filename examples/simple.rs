use caduceus_term::proxy::{run_proxy, ChildMode};
use caduceus_term::ProxyBuilder;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let config = ProxyBuilder::new("cat")
        .child_mode(ChildMode::Piped)
        .build();

    let status = run_proxy(config).await?;
    std::process::exit(status.code().unwrap_or(1));
}
