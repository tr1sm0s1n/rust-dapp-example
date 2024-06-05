use alloy::{
    providers::{ProviderBuilder, RootProvider},
    rpc::types::eth::TransactionReceipt,
    sol,
    transports::http::{Client, Http},
};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use eyre::Result;
use serde::{Deserialize, Serialize};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use Cert::CertInstance;

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
    Cert,
    "utils/Cert.json"
);

async fn instance() -> Result<CertInstance<Http<Client>, RootProvider<Http<Client>>>> {
    let rpc_url = "http://127.0.0.1:8545".parse()?;

    // Create a provider with the HTTP transport using the `reqwest` crate.
    let provider = ProviderBuilder::new().on_http(rpc_url);
    let contract = Cert::new(
        "0x5FbDB2315678afecb367f032d93F642f64180aa3".parse()?,
        provider,
    );

    Ok(contract)
}

#[tokio::main]
async fn main() {
    let cert = instance().await.unwrap();

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

fn app(cert: CertInstance<Http<Client>, RootProvider<Http<Client>>>) -> Router {
    // build our application with multiple routes
    Router::new()
        .route("/", get(home))
        .route("/issue", post(issue_certificate))
        .route("/fetch/:id", get(fetch_certificate))
        .layer(TraceLayer::new_for_http())
        .with_state(cert)
}

async fn home() -> &'static str {
    "Hello, World!"
}

async fn issue_certificate(
    State(cert): State<CertInstance<Http<Client>, RootProvider<Http<Client>>>>,
    Json(input): Json<Certificate>,
) -> Result<Json<TransactionReceipt>, (StatusCode, String)> {
    let builder = cert.issue(input.id, input.name, input.course, input.grade, input.date);
    let receipt = builder
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
    State(cert): State<CertInstance<Http<Client>, RootProvider<Http<Client>>>>,
) -> Result<Json<Certificate>, (StatusCode, String)> {
    let result = cert
        .Certificates(id.clone())
        .call()
        .await
        .map_err(internal_error)?;
    let certificate = Certificate {
        id,
        name: result.name,
        course: result.course,
        grade: result.grade,
        date: result.date,
    };

    Ok(Json(certificate))
}

fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}
