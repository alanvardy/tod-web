use crate::Link;
use askama_axum::Template;
use axum::{response::Html, routing::get, Router};

pub fn routes() -> Router {
    Router::new().route("/usage", get(usage))
}

#[derive(Template)]
#[template(path = "usage.html")]
struct UsageTemplate {
    title: String,
    navigation: Vec<Link>,
}

async fn usage() -> Html<String> {
    let usage = UsageTemplate {
        title: "Usage".into(),
        navigation: crate::get_nav(),
    };

    Html(usage.render().unwrap())
}
