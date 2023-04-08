use reqwest::{header, Error};
use serde::Deserialize;
use std::collections::HashMap;

pub struct HttpClient {
    base_url: String,
    headers: header::HeaderMap,
    query: Vec<(String, String)>,
    pub client: reqwest::Client,
}

impl HttpClient {
    pub fn new(
        base_url: &str,
        headers: header::HeaderMap,
        query: Vec<(String, String)>,
    ) -> Result<Self, Error> {
        let client = reqwest::Client::builder().build()?;
        Ok(Self {
            base_url: base_url.to_owned(),
            headers,
            client,
            query,
        })
    }

    pub async fn get<T>(
        &self,
        path: &str,
        extra_query: Option<HashMap<String, String>>,
    ) -> Result<T, Error>
    where
        T: for<'de> Deserialize<'de>,
    {
        let mut query = self.query.clone();
        if let Some(extra_query) = extra_query {
            query.extend(extra_query);
        }
        let url = format!("{}{}", self.base_url, path);
        let response = self
            .client
            .get(&url)
            .headers(self.headers.clone())
            .query(&query)
            .send()
            .await?;
        let json = response.json::<T>().await?;
        Ok(json)
    }

    // pub async fn get<T>(&self, path: &str) -> Result<T, Error>
    // where
    //     T: for<'de> Deserialize<'de>,
    // {
    //     let url = format!("{}{}", self.base_url, path);
    //     let response = self
    //         .client
    //         .get(&url)
    //         .headers(self.headers.clone())
    //         .query(&self.query)
    //         .send()
    //         .await?;
    //     let json = response.json::<T>().await?;
    //     Ok(json)
    // }

    // pub async fn post<T>(&self, path: &str, body: &T) -> Result<Response, Error>
    // where
    //     T: Serialize,
    // {
    //     let url = format!("{}{}", self.base_url, path);
    //     let resp = self.client
    //         .post(&url)
    //         .headers(self.headers.clone())
    //         .json(body)
    //         .send()
    //         .await?;
    //
    //     Ok(resp)
    // }
}
