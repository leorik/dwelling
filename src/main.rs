use axum::Router;
use axum::extract::{Query, State};
use axum::response::Html;
use axum::routing::get;
use handlebars::Handlebars;
use serde::ser::SerializeStruct;
use serde::{Deserialize, Serialize, Serializer};
use std::net::Ipv4Addr;
use std::sync::Arc;
use tokio::net::TcpListener;
use tower_http::services::{ServeDir, ServeFile};

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
    let state = Arc::new(CommonState {
        templater: init_templating(),
    });

    let not_found_handler = ServeFile::new("./templates/not_found.html");
    let app = Router::new()
        .route("/", get(|| async { "Stub" }))
        .route("/hello", get(render_hello))
        .nest_service(
            "/assets",
            ServeDir::new("assets").fallback(not_found_handler.clone()),
        )
        .with_state(state)
        .fallback_service(not_found_handler);

    let listener = TcpListener::bind((Ipv4Addr::LOCALHOST, 8080))
        .await
        .unwrap();
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

#[derive(Deserialize)]
struct HelloStruct {
    name: Option<String>,
}

impl Serialize for HelloStruct {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut builder = serializer.serialize_struct("HelloStruct", 1)?;

        let name = match self.name {
            Some(ref n) => n,
            None => "World",
        };
        builder.serialize_field("name", name)?;

        builder.end()
    }
}

async fn render_hello(
    Query(hello): Query<HelloStruct>,
    State(state): State<Arc<CommonState>>,
) -> Html<String> {
    Html::from(state.get_templater().render("index", &hello).unwrap())
}
