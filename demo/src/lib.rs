use async_graphql::{http::GraphiQLSource, EmptyMutation, EmptySubscription, Schema};
use async_graphql_axum::GraphQL;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{response::Html, Router};
use tower_service::Service;
use worker::{event, Context, Env, Result};

mod starwars;
use starwars::{QueryRoot, StarWars};

#[worker::send]
pub async fn graphiql() -> impl IntoResponse {
    Html(GraphiQLSource::build().endpoint("/").finish())
}

#[event(fetch)]
async fn fetch(
    req: axum::http::Request<worker::Body>,
    _env: Env,
    _ctx: Context,
) -> Result<axum::http::Response<axum::body::Body>> {
    let schema = Schema::build(QueryRoot, EmptyMutation, EmptySubscription)
        .data(StarWars::new())
        .finish();

    let mut router = Router::new().route("/", get(graphiql).post_service(GraphQL::new(schema)));

    Ok(router.call(req).await.unwrap())
}
