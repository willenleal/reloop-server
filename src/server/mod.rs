use axum::{routing::post, Router};

use dotenv::dotenv;
use std::net::SocketAddr;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod config;
mod error;
mod graphql;
mod http;

use crate::server::graphql::schema::{build_schema, graphql_handler, graphql_playground};

pub async fn init() {
    dotenv().expect("Missing .env file");

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "reloop=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let schema = build_schema();

    let app = Router::new()
        .route(
            "/api/graphql",
            post(graphql_handler).get(graphql_playground),
        )
        .layer(TraceLayer::new_for_http())
        .with_state(schema);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    tracing::debug!("listening on {addr}");

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
