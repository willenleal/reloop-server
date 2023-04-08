use super::query::MovieQuery;
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql::{EmptyMutation, EmptySubscription, MergedObject, Schema};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use async_graphql::extensions::ApolloTracing;
use axum::{
    extract::State,
    response::{Html, IntoResponse},
};
use std::env;
// use clap::Parser;
//
// use crate::server::config::Config;
use crate::server::http::HttpClient;
use reqwest::header;

#[derive(MergedObject, Default)]
pub struct RootQuery(MovieQuery);

pub type AppSchema = Schema<RootQuery, EmptyMutation, EmptySubscription>;

pub async fn graphql_handler(schema: State<AppSchema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

pub async fn graphql_playground() -> impl IntoResponse {
    Html(playground_source(GraphQLPlaygroundConfig::new(
        "/api/graphql",
    )))
}

pub fn build_schema() -> AppSchema {
    let api_key = env::var("tmdb_key").expect("Missing tmdb_key");

    let headers = header::HeaderMap::new();
    let query = vec![("api_key".to_owned(), api_key)];
    let http_client = HttpClient::new("https://api.themoviedb.org/3", headers, query).unwrap();

    Schema::build(RootQuery::default(), EmptyMutation, EmptySubscription)
        .data(http_client)
        .extension(ApolloTracing)
        .finish()
}
