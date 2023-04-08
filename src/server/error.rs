use anyhow::anyhow;
use async_graphql::{Error, ErrorExtensions};
use reqwest::StatusCode;

pub fn graphql_error(message: String, code: StatusCode) -> Error {
    anyhow!(message).extend_with(|_err, e| {
        e.set("code", code.as_u16());
        e.set("message", code.to_string());
    })
}

pub fn as_graphql_error<E: std::fmt::Display>(
    message: &'static str,
    code: StatusCode,
) -> Box<dyn Fn(E) -> Error + Send + Sync> {
    Box::new(move |err| {
        anyhow!(message).extend_with(|_err, e| {
            e.set("code", code.as_u16());
            e.set("message", code.to_string());
            e.set("reason", err.to_string());
        })
    })
}
