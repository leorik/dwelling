use crate::CommonState;
use axum::extract::{Query, State};
use axum::response::Html;
use serde::ser::SerializeStruct;
use serde::{Deserialize, Serialize, Serializer};
use std::sync::Arc;
use tracing::instrument;

#[derive(Deserialize, Debug)]
pub struct HelloStruct {
    pub name: Option<String>,
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

#[instrument(skip(state))]
pub async fn render_hello(
    Query(hello): Query<HelloStruct>,
    State(state): State<Arc<CommonState>>,
) -> Html<String> {
    Html::from(state.get_templater().render("index", &hello).unwrap())
}
