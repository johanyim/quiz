#![allow(warnings)]

mod quiz;
mod question;
mod error;
mod utils;

use askama::Template;
// use turf;

use question::*;
use quiz::*;

use anyhow::Result;

use axum::{
    // extract::{Path, Query, State}, 

    response::{Response,IntoResponse, Html}, 
    routing::get, 
    http::{
        StatusCode, 
        HeaderMap
    },
    // Json, 
    Router
};
use serde::{
    Serialize, 
    // Deserialize,
};
use tower_http::services::ServeDir;


#[tokio::main]
async fn main() -> Result<()> {

    let app: Router = Router::new()
        .route("/", get(index))
        .nest_service("/static", ServeDir::new("static"))
        .merge(question_router());
        // .route("/", get(handler))

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000")
        .await?;

    axum::serve(listener, app)
        .await
        .unwrap();

    return Ok(())
}

fn question_router() -> Router {
    Router::new()
        .route("/previous", get(previous_question))
        .route("/next", get(next_question))
}


async fn previous_question() -> impl IntoResponse {

    // let question = question::MultipleChoiceRadio::new("What is the capital of China?", 
    //                                                   vec!["Hong kong", "Beijing", "Guangzhou", "Taiwan", "Thailand"]
    //                                                   .into_iter().map(String::from)
    //                                                   .collect::<Vec<String>>(), 
    //                                                   1);
    let question = question::Integer::new("How many protons are in a hydrogen nucleus?", 1);

    // (headers, HtmlTemplate(question))
    HtmlTemplate(question)
}

async fn next_question() -> impl IntoResponse {


    let question = question::MultipleChoiceRadio::new("What is the symbol for Helium?", 
                                                      vec!["H", "H2O", "He", "Hl", "he"]
                                                      .into_iter().map(String::from)
                                                      .collect::<Vec<String>>(), 
                                                      1);
    // let question = question::Integer::new("How many protons are in a hydrogen nucleus?", 1);
    // let question = question::Text::new("What is the symbol for Hydrogen?", "H");

    // (headers, HtmlTemplate(question))
    HtmlTemplate(question)
}



async fn index() -> impl IntoResponse {
    turf::style_sheet!("styles/style.scss");
    let index = IndexTemplate {};
    HtmlTemplate(index)
}


#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate;


struct HtmlTemplate<T>(T);
impl<T> IntoResponse for HtmlTemplate<T>
where
    T: Template,
{
    fn into_response(self) -> Response {
        // Attempt to render the template with askama
        match self.0.render() {
            // If we're able to successfully parse and aggregate the template, serve it
            Ok(html) => Html(html).into_response(),
            // If we're not, return an error or some bit of fallback HTML
            Err(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to render template. Error: {}", err),
                )
                .into_response(),
        }
    }

}

