mod config;
mod hello;
mod model;
mod observability;

use axum::Router;
use axum::extract::MatchedPath;
use axum::http::{Request, StatusCode};
use axum::routing::get;
use handlebars::Handlebars;
use std::net::Ipv4Addr;
use std::sync::Arc;
use tokio::net::TcpListener;
use tower_http::services::{ServeDir, ServeFile};
use tower_http::set_status::SetStatus;
use tower_http::trace::TraceLayer;
use tracing::{info, trace_span};

pub struct CommonState {
    templater: Handlebars<'static>,
}

impl CommonState {
    pub fn get_templater(&self) -> &Handlebars<'static> {
        &self.templater
    }
}

#[tokio::main]
async fn main() {
    observability::init();

    let config = config::load();

    let state = Arc::new(CommonState {
        templater: init_templating(),
    });

    let not_found_handler = SetStatus::new(
        ServeFile::new("./templates/not_found.html"),
        StatusCode::NOT_FOUND,
    );

    let app = Router::new()
        .route("/", get(|| async { "Stub" }))
        .route("/hello", get(hello::render_hello))
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
