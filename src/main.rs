use askama::Template;
use axum::{
    extract::{self},
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    routing::get,
    Router,
};
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::net::SocketAddr;
use tracing_subscriber;

lazy_static! {
    static ref D: HashMap<&'static str, &'static str> =
        HashMap::from([("en.key", "en value"), ("fr.key", "fr value")]);
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let app = Router::new().route("/:lng", get(index));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn index(extract::Path(lng): extract::Path<String>) -> impl IntoResponse {
    let template: IndexTemplate = IndexTemplate {
        lng,
        t: |l: &String, k: &str| (D.get(format!("{l}.{k}").as_str()).unwrap()).to_string(),
    };
    HtmlTemplate(template)
}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    lng: String,
    t: fn(&String, &str) -> String,
}

struct HtmlTemplate<T>(T);

impl<T> IntoResponse for HtmlTemplate<T>
where
    T: Template,
{
    fn into_response(self) -> Response {
        match self.0.render() {
            Ok(html) => Html(html).into_response(),
            Err(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to render template. Error: {err}"),
            )
                .into_response(),
        }
    }
}
