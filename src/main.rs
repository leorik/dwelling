mod config;
mod database;
mod hello;
mod model;
mod observability;
mod thread;

use std::error::Error;
use axum::Router;
use axum::extract::MatchedPath;
use axum::http::{Request, StatusCode};
use axum::routing::get;
use handlebars::Handlebars;
use sqlx::{Pool, Postgres};
use std::net::Ipv4Addr;
use std::sync::Arc;
use axum::response::Html;
use serde::{Deserialize, Serialize};
use thiserror::__private::AsDisplay;
use tokio::net::TcpListener;
use tower_http::services::{ServeDir, ServeFile};
use tower_http::set_status::SetStatus;
use tower_http::trace::TraceLayer;
use tracing::{info, trace_span};

pub struct CommonState {
    templater: Handlebars<'static>,
    connection_pool: Pool<Postgres>,
}

impl CommonState {
    pub fn get_templater(&self) -> &Handlebars<'static> {
        &self.templater
    }

    pub fn get_db_connection_pool(&self) -> &Pool<Postgres> {
        &self.connection_pool
    }
}

#[tokio::main]
async fn main() {
    observability::init();

    let config = config::load();

    let connection_pool = match database::init_database(&config).await {
        Ok(pool) => pool,
        Err(err) => {
            panic!("{err}")
        }
    };

    let state = Arc::new(CommonState {
        templater: init_templating(),
        connection_pool,
    });

    let not_found_handler = SetStatus::new(
        ServeFile::new("./templates/not_found.html"),
        StatusCode::NOT_FOUND,
    );

    let app = Router::new()
        .route("/", get(|| async { "Stub" }))
        .route("/hello", get(hello::render_hello))
        .nest("/thread", thread::build_router())
        .nest_service(
            "/assets",
            ServeDir::new("assets").fallback(not_found_handler.clone()),
        )
        .with_state(state)
        .fallback_service(not_found_handler)
        .layer(
            TraceLayer::new_for_http().make_span_with(|request: &Request<_>| {
                let matched_path = request
                    .extensions()
                    .get::<MatchedPath>()
                    .map(MatchedPath::as_str);

                trace_span!(
                    "http_request",
                    method = ?request.method(),
                    matched_path,
                )
            }),
        );

    let listener = TcpListener::bind((Ipv4Addr::LOCALHOST, config.app_port))
        .await
        .unwrap();

    info!(
        "Starting Dwelling on address: {}",
        listener.local_addr().unwrap()
    );

    axum::serve(listener, app).await.unwrap();
}

fn init_templating() -> Handlebars<'static> {
    let mut templater = Handlebars::new();
    templater.set_strict_mode(true);

    // TODO modularize?
    templater
        .register_template_file("index", "./templates/index.hbs")
        .unwrap();

    templater
}

#[derive(Serialize)]
struct ErrorDisplay {
    name: String
}
pub fn render_error(templater: &Handlebars<'static>, error: &dyn Error) -> Html<String> {
    let display = ErrorDisplay { name: error.to_string() };
    let rendered = templater.render("error", &display).unwrap_or_else(|e| format!("Encountered error while rendering error: {e}"));
    Html::from(rendered)
}
