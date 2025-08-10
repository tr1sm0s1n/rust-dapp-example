use alloy::{
    primitives::Address,
    providers::{Provider, ProviderBuilder},
    rpc::{
        client::WsConnect,
        types::eth::{BlockNumberOrTag, Filter},
    },
    sol,
    sol_types::SolEvent,
};
use eyre::Result;
use futures::stream::StreamExt;

sol!(
    #[sol(rpc)]
    "../foundry/src/Cert.sol"
);

#[tokio::main]
async fn main() -> Result<()> {
    let rpc_url = "ws://127.0.0.1:8545";

    // Create the provider.
    let ws = WsConnect::new(rpc_url);
    let provider = ProviderBuilder::new().connect_ws(ws).await?;

    let filter = Filter::new()
        .address(
            "0x5FbDB2315678afecb367f032d93F642f64180aa3"
                .parse::<Address>()
                .unwrap(),
        )
        // By specifying an `event` or `event_signature` we listen for a specific event of the contract.
        .event(Cert::Issued::SIGNATURE)
        .from_block(BlockNumberOrTag::Latest);

    // Subscribe to logs.
    let sub = provider.subscribe_logs(&filter).await?;
    let mut stream = sub.into_stream();

    println!("Listening for events...");
    println!("-----------------------");

    while let Some(log) = stream.next().await {
        println!("Certificate issued!!");
        println!("--------------------");
        let Cert::Issued { course, id, grade } = log.log_decode()?.inner.data;
        println!("Course: \x1b[34m{course}\x1b[0m");
        println!("ID: \x1b[34m{id}\x1b[0m");
        println!("Grade: \x1b[34m{grade}\x1b[0m");
        println!("Raw Log: \x1b[32m{log:?}\x1b[0m");
        println!("--------------------");
    }

    Ok(())
}
