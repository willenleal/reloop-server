use std::collections::HashMap;

use async_graphql::futures_util::try_join;
use async_graphql::{Context, Object, Result, SimpleObject};
use reqwest::StatusCode;
use serde::Deserialize;

use crate::server::{error::{as_graphql_error, graphql_error}, http::HttpClient};

#[derive(Default)]
pub struct MovieQuery;

#[derive(Deserialize, SimpleObject)]
#[graphql(name = "Movie")]
struct Movie {
    poster_path: Option<String>,
    overview: String,
    release_date: String,
    id: usize,
    title: String,
    backdrop_path: Option<String>,
    vote_average: f32,
}

#[derive(Deserialize, SimpleObject)]
#[graphql(name = "Pagination")]
struct Paginated<T: async_graphql::OutputType> {
    page: usize,
    results: Vec<T>,
    total_results: usize,
    total_pages: usize,
}

#[derive(SimpleObject)]
struct MoviesHome {
    now_playing: Vec<Movie>,
    popular: Vec<Movie>,
    top_rated: Vec<Movie>,
    upcoming: Vec<Movie>,
}

#[Object]
impl MovieQuery {
    async fn movies_now_playing(
        &self,
        ctx: &Context<'_>,
        page: Option<usize>,
    ) -> Result<Paginated<Movie>> {
        let mut query: HashMap<String, String> = HashMap::new();

        if let Some(p) = page {
            query.insert("page".into(), p.to_string());
        }

        let http_client = ctx.data::<HttpClient>()?;

        let movies: Paginated<Movie> = http_client
            .get("/movie/now_playing", Some(query))
            .await
            .map_err(as_graphql_error(
                "Failed to fetch resources",
                StatusCode::INTERNAL_SERVER_ERROR,
            ))?;

        Ok(movies)
    }

    async fn movies_popular(
        &self,
        ctx: &Context<'_>,
        page: Option<usize>,
    ) -> Result<Paginated<Movie>> {
        let mut query: HashMap<String, String> = HashMap::new();

        if let Some(p) = page {
            query.insert("page".into(), p.to_string());
        }

        let http_client = ctx.data::<HttpClient>()?;

        let movies: Paginated<Movie> = http_client
            .get("/movie/popular", Some(query))
            .await
            .map_err(as_graphql_error(
                "Failed to fetch resources",
                StatusCode::INTERNAL_SERVER_ERROR,
            ))?;

        Ok(movies)
    }

    async fn movies_top_rated(
        &self,
        ctx: &Context<'_>,
        page: Option<usize>,
    ) -> Result<Paginated<Movie>> {
        let mut query: HashMap<String, String> = HashMap::new();

        if let Some(p) = page {
            query.insert("page".into(), p.to_string());
        }

        let http_client = ctx.data::<HttpClient>()?;

        let movies: Paginated<Movie> = http_client
            .get("/movie/top_rated", Some(query))
            .await
            .map_err(as_graphql_error(
                "Failed to fetch resources",
                StatusCode::INTERNAL_SERVER_ERROR,
            ))?;

        Ok(movies)
    }

    async fn movies_upcoming(
        &self,
        ctx: &Context<'_>,
        page: Option<usize>,
    ) -> Result<Paginated<Movie>> {
        let mut query: HashMap<String, String> = HashMap::new();

        if let Some(p) = page {
            query.insert("page".into(), p.to_string());
        }

        let http_client = ctx.data::<HttpClient>()?;

        let movies: Paginated<Movie> = http_client
            .get("/movie/upcoming", Some(query))
            .await
            .map_err(as_graphql_error(
                "Failed to fetch resources",
                StatusCode::INTERNAL_SERVER_ERROR,
            ))?;

        Ok(movies)
    }

    async fn movies_home(&self, ctx: &Context<'_>) -> Result<MoviesHome> {
        let (now_playing, popular, top_rated, upcoming) = try_join!(
            self.movies_now_playing(ctx, None),
            self.movies_popular(ctx, None),
            self.movies_top_rated(ctx, None),
            self.movies_upcoming(ctx, None),
        )
        .map_err(|e| graphql_error(e.message, StatusCode::INTERNAL_SERVER_ERROR))?;

        Ok(MoviesHome {
            now_playing: now_playing.results,
            popular: popular.results,
            top_rated: top_rated.results,
            upcoming: upcoming.results,
        })
    }
}
