use Cert::CertInstance;
use alloy::{
    providers::{
        Identity, Provider, ProviderBuilder, RootProvider,
        fillers::{BlobGasFiller, ChainIdFiller, FillProvider, GasFiller, JoinFill, NonceFiller},
    },
    rpc::types::eth::TransactionReceipt,
    sol,
};
use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    routing::{get, post},
};
use eyre::Result;
use serde::{Deserialize, Serialize};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

type Instance = CertInstance<
    FillProvider<
        JoinFill<
            Identity,
            JoinFill<GasFiller, JoinFill<BlobGasFiller, JoinFill<NonceFiller, ChainIdFiller>>>,
        >,
        RootProvider,
    >,
>;

#[derive(Debug, Deserialize, Serialize)]
struct Certificate {
    id: String,
    name: String,
    course: String,
    grade: String,
    date: String,
}

sol!(
    #[sol(rpc)]
    "../foundry/src/Cert.sol"
);

async fn instance_builder() -> Result<Instance> {
    let rpc_url = "http://127.0.0.1:8545".parse()?;

    // Create a provider with the HTTP transport using the `reqwest` crate.
    let provider = ProviderBuilder::new().connect_http(rpc_url);
    let contract = Cert::new(
        "0x5FbDB2315678afecb367f032d93F642f64180aa3".parse()?,
        provider,
    );

    Ok(contract)
}

#[tokio::main]
async fn main() {
    let cert = instance_builder().await.unwrap();

    // logging middleware
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "rust_dapp_api=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // run it
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    tracing::debug!("Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app(cert)).await.unwrap();
}

fn app(instance: Instance) -> Router {
    // build our application with multiple routes
    Router::new()
        .route("/", get(home))
        .route("/issue", post(issue_certificate))
        .route("/fetch/{id}", get(fetch_certificate))
        .layer(TraceLayer::new_for_http())
        .with_state(instance)
}

async fn home() -> &'static str {
    "Hello, World!"
}

async fn issue_certificate(
    State(instance): State<Instance>,
    Json(input): Json<Certificate>,
) -> Result<Json<TransactionReceipt>, (StatusCode, String)> {
    // Call builder for the issue function.
    let builder = instance.issue(input.id, input.name, input.course, input.grade, input.date);

    // Get unlocked accounts from the network.
    let accounts = instance
        .provider()
        .get_accounts()
        .await
        .map_err(internal_error)?;

    // Use the first account to send transaction.
    let receipt = builder
        .from(accounts[0])
        .send()
        .await
        .map_err(internal_error)?
        .get_receipt()
        .await
        .map_err(internal_error)?;

    Ok(Json(receipt))
}

async fn fetch_certificate(
    Path(id): Path<String>,
    State(instance): State<Instance>,
) -> Result<Json<Certificate>, (StatusCode, String)> {
    // Fetch certificate corresponding to 'id'.
    let result = instance
        .Certificates(id.clone())
        .call()
        .await
        .map_err(internal_error)?;

    // Create a new variable of type 'Certificate' for payload.
    let certificate = Certificate {
        id,
        name: result._0,
        course: result._1,
        grade: result._2,
        date: result._3,
    };

    Ok(Json(certificate))
}

fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}
