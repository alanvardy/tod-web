use std::sync::Arc;

use axum::{response::Html, routing::get, Extension, Router};
use minijinja::{context, path_loader, Environment, Value};

#[derive(Clone)]
struct State {
    env: Arc<Environment<'static>>,
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    // build our application with a route
    let mut env = Environment::new();
    env.set_loader(path_loader("templates"));

    let state = Arc::new(State { env: Arc::new(env) });

    let router = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        // `POST /users` goes to `create_user`
        .layer(Extension(state));
    Ok(router.into())
}

async fn root(Extension(state): Extension<Arc<State>>) -> Html<String> {
    let context = context!(name => "Axum");
    render_template(state, "index.html", context).await
}

async fn render_template(state: Arc<State>, name: &str, context: Value) -> Html<String> {
    let tmpl = state.env.get_template(name).unwrap();
    Html(tmpl.render(context).unwrap())
}
