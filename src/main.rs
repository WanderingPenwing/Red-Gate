use pulldown_cmark::{Parser, html, Event, Tag, HeadingLevel};
use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use std::fs;

fn slugify(text: &str) -> String {
    text.to_lowercase()
        .replace(" ", "-")
        .replace(|c: char| !c.is_alphanumeric() && c != '-', "")
}

fn markdown_to_html(markdown_content: &str) -> String {
    let parser = Parser::new(markdown_content);

    let mut events = Vec::new();
    let mut in_header = false;
    let mut header_text = String::new();

    for event in parser {
        match &event {
            Event::Start(Tag::Heading(HeadingLevel::H6, ..)) => {
                in_header = true;
                header_text.clear();
            }
            Event::End(Tag::Heading(HeadingLevel::H6, ..)) => {
                in_header = false;
                let slug = slugify(&header_text);
                events.push(Event::Html(format!("<p id=\"{}\"><br></p>", slug).into()));
            }
            Event::Text(text) => {
                if in_header {
                    header_text.push_str(text);
                } else {
                    events.push(event.clone());
                }
            }
            _ => {
                events.push(event.clone());
            }
        }
    }

    let mut html_renderer = String::new();
    html::push_html(&mut html_renderer, events.into_iter());
    html_renderer
}

async fn serve_markdown(file_path: &str) -> impl Responder {
    // Read the Markdown file
    let markdown_content = fs::read_to_string(file_path).unwrap_or_else(|_| String::from("Error reading markdown file"));

    // Convert Markdown to HTML
    let html_content = markdown_to_html(&markdown_content);

    // Create the common HTML wrapper
    let common_html = fs::read_to_string("pages/common.html").unwrap_or_else(|_| String::from("Error reading html file"));

    // Replace #CONTENT# placeholder with the actual Markdown HTML content
    let page_html = common_html.replace("#CONTENT#", &html_content);

    HttpResponse::Ok().content_type("text/html").body(page_html)
}


async fn summary() -> impl Responder {
    serve_markdown("pages/summary.md").await
}

async fn tools() -> impl Responder {
    serve_markdown("pages/tools.md").await
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(summary))  // Route for root
            .route("/tools", web::get().to(tools)) // Route for tools
            .service(actix_files::Files::new("/assets", "./assets").show_files_listing())
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}


