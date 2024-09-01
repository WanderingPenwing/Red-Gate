use pulldown_cmark::{Parser, html};
use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use std::fs;

fn markdown_to_html(markdown_content: &str) -> String {
    let mut html_output = String::new();
    let parser = Parser::new(markdown_content);
    html::push_html(&mut html_output, parser);
    html_output
}

async fn serve_markdown(file_path: &str) -> impl Responder {
    // Read the Markdown file
    let markdown_content = fs::read_to_string(file_path).unwrap_or_else(|_| String::from("Error reading file"));

    // Convert Markdown to HTML
    let html_content = markdown_to_html(&markdown_content);

    // Create the common HTML wrapper
    let common_html = include_str!("common.html");

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


