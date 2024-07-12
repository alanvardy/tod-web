use std::sync::Arc;

use axum::{response::Html, routing::get, Extension, Router};
use minijinja::{context, path_loader, Environment, Value};

#[derive(Clone)]
struct State {
    env: Arc<Environment<'static>>,
}

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    // build our application with a route
    let mut env = Environment::new();
    env.set_loader(path_loader("templates"));

    let state = Arc::new(State { env: Arc::new(env) });

    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        // `POST /users` goes to `create_user`
        .layer(Extension(state));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root(Extension(state): Extension<Arc<State>>) -> Html<String> {
    let context = context!(name => "Axum");
    render_template(state, "index.html", context).await
}

async fn render_template(state: Arc<State>, name: &str, context: Value) -> Html<String> {
    let tmpl = state.env.get_template(name).unwrap();
    Html(tmpl.render(context).unwrap())
}
