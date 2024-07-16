use crate::Link;
use askama_axum::Template;
use axum::{response::Html, routing::get, Router};

pub fn routes() -> Router {
    Router::new().route("/", get(index))
}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    title: String,
    navigation: Vec<Link>,
}

async fn index() -> Html<String> {
    let index = IndexTemplate {
        title: "Tod".into(),
        navigation: crate::get_nav(),
    };

    Html(index.render().unwrap())
}
