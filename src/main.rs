//! Run with
//!
//! ```not_rust
//! cargo run -p example-form
//! ```

use std::path::Path as OsPath;
use askama::Template;
use axum::response::IntoResponse;
use axum::{
    Router, ServiceExt,
    extract::Request,
    extract::{Form, Path},
    http::StatusCode,
    response::Html,
    routing::get,
};
use git2::Repository;
use serde::Deserialize;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
mod git;
mod html;
use git::repository::Repository as Repo;
use html::html::{RepoBase, Tree};
use tower_http::normalize_path::NormalizePathLayer;
use tower_http::services::ServeDir;
use tower_layer::Layer;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| format!("{}=debug", env!("CARGO_CRATE_NAME")).into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // build our application with some routes
    let app = NormalizePathLayer::trim_trailing_slash().layer(app());

    // run it
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, ServiceExt::<Request>::into_make_service(app))
        .await
        .unwrap();
}

fn app() -> Router {
    let git = Router::new()
        .route("/{id}", get(index))
        .route("/{id}/{*filepath}", get(blob));

    Router::new()
        .nest_service("/static/", ServeDir::new("static"))
        .nest("/git", git)
        .route("/", get(show_form).post(accept_form))
}

async fn show_form() -> Html<&'static str> {
    Html(
        r#"
        <!doctype html>
        <html>
            <head></head>
            <body>
                <form action="/" method="post">
                    <label for="name">
                        Enter your name:
                        <input type="text" name="name">
                    </label>

                    <label>
                        Enter your email:
                        <input type="text" name="email">
                    </label>

                    <input type="submit" value="Subscribe!">
                </form>
            </body>
        </html>
        "#,
    )
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
struct Input {
    name: String,
    email: String,
}

async fn accept_form(Form(input): Form<Input>) -> Html<String> {
    dbg!(&input);
    Html(format!("email='{}'\nname='{}'\n", input.email, input.name))
}
async fn index(Path(name): Path<String>) -> (StatusCode, Html<String>) {
    dbg!(&name);
    let path_template = format!(
        "{}/{}",
        std::env::var("HOME").expect("Home variable doesn't exist"),
        name
    );
    let path = OsPath::new(&path_template);
    let repo = match Repository::open(path) {
        Ok(repo) => repo,
        Err(_e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Html(format!("Faild to Open repo")),
            );
        }
    };
    let index: Vec<_> = repo
        .index()
        .expect("Couldn't get the repo index")
        .iter()
        .collect();
    dbg!(index.len());
    let html_list: Vec<String> = index
        .into_iter()
        .map(|item| String::from_utf8(item.path).unwrap())
        .collect();
    dbg!(&html_list);

    let x = Tree {
        base: RepoBase {
            repo: name.clone(), 
            rev: "Index".to_owned(),
            active: "tree",
            // TODO!
            description: "Description to be defined later.".to_owned(),
        },
        path_parts: vec![],
        parent_path: None,
        entries: vec![],
    };
    dbg!(&name);
    (StatusCode::OK, Html(x.render().unwrap()))
}

async fn blob(Path((id, blob)): Path<(String, String)>) -> impl IntoResponse {
    let repo = Repo::open(id).unwrap();
    let tree = repo.tree().unwrap();
    let object = tree.get_name(&blob).unwrap();
    let id = object.id();
    let blob = repo.repo.find_blob(id).unwrap();
    dbg!(blob.content());
    let content = String::from_utf8(blob.content().to_vec());
    dbg!(repo.tree_iter());
    content.unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::Body,
        http::{self, Request, StatusCode},
    };
    use http_body_util::BodyExt;
    use tower::ServiceExt; // for `call`, `oneshot`, and `ready` // for `collect`

    #[tokio::test]
    async fn test_get() {
        let app = app();

        let response = app
            .oneshot(Request::get("/").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = response.into_body().collect().await.unwrap().to_bytes();
        let body = std::str::from_utf8(&body).unwrap();

        assert!(body.contains(r#"<input type="submit" value="Subscribe!">"#));
    }

    #[tokio::test]
    async fn test_post() {
        let app = app();

        let response = app
            .oneshot(
                Request::post("/")
                    .header(
                        http::header::CONTENT_TYPE,
                        mime::APPLICATION_WWW_FORM_URLENCODED.as_ref(),
                    )
                    .body(Body::from("name=foo&email=bar@axum"))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = response.into_body().collect().await.unwrap().to_bytes();
        let body = std::str::from_utf8(&body).unwrap();

        assert_eq!(body, "email='bar@axum'\nname='foo'\n");
    }
}
