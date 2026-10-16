//! Run with
//!
//! ```not_rust
//! cargo run -p example-form
//! ```

use axum::{extract::{Form,Path}, response::Html, routing::get, Router, http::StatusCode};
use axum::response::IntoResponse;
use serde::Deserialize;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use git2::Repository;

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
    let app = app();

    // run it
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await;
}

fn app() -> Router {
    let git = Router::new()
        .route("/{id}", get(git))
        .route("/{id}/{*filepath}", get(file));

    Router::new()
        .nest("/git",git)
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
use std::path::Path as OsPath;
async fn git(Path(name): Path<String>)-> (StatusCode, Html<String>){
    let path_template = format!("{}/{}",std::env::var("HOME").expect("Home variable doesn't exist"),name);
    let path = OsPath::new(&path_template);
    let repo = match Repository::open(path){
        Ok(repo) => repo,
        Err(e) => {return (StatusCode::INTERNAL_SERVER_ERROR, Html(format!("Faild to Open repo")));},
    };
    let mut index: Vec<_> = repo.index().expect("Couldn't get the repo index").iter().collect();
    dbg!(index.len());
    let html_list: String = index.into_iter().map(|item| {
        let tmp = String::from_utf8(item.path).unwrap();

   dbg!(&tmp);
   println!("This line executed");
        format!("<li>{}</li>\n",&tmp)
    }).collect();
    dbg!(&html_list);

    dbg!(&name);
    (StatusCode::OK, Html(format!
                          (r#"<!DOCTYPE html>
            <html lang="en">
              <head>
                  <meta charset="UTF-8">
                      <meta name="viewport" content="width=device-width, initial-scale=1.0">
                          <meta http-equiv="X-UA-Compatible" content="ie=edge">
                              <title>HTML 5 Boilerplate</title>
                                  <link rel="stylesheet" href="style.css">
                                    </head>
                                      <body>

                                            {}
                                            </body>
                                            </html>"#,html_list
                )))

}



async fn file() -> impl IntoResponse{}



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
