use axum::Router;
use serde::Serialize;

mod configuration;
mod index;

#[derive(Serialize)]
struct Link {
    name: String,
    href: String,
}

fn router() -> Router {
    Router::new()
        // Routes
        .merge(index::routes())
        .merge(configuration::routes())
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    Ok(router().into())
}

fn get_nav() -> Vec<Link> {
    vec![
        Link {
            href: "/".into(),
            name: "Tod".into(),
        },
        Link {
            href: "/configuration".into(),
            name: "Configuration".into(),
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum_test::TestServer;

    #[tokio::test]
    async fn test_index() {
        // you can replace this Router with your own app

        let server = TestServer::new(router()).unwrap();
        // Get the request.
        let response = server.get("/").await;

        assert!(response
            .text()
            .contains("An unofficial Todoist CLI program. "))
    }

    #[tokio::test]
    async fn test_configuration() {
        // you can replace this Router with your own app

        let server = TestServer::new(router()).unwrap();
        // Get the request.
        let response = server.get("/configuration").await;

        assert!(response
            .text()
            .contains("Data is stored at $XDG_CONFIG_HOME/tod.cfg. This defaults to:"))
    }
}
