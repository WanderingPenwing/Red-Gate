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
    let common_html = r#"
    <!DOCTYPE html>
    <html lang="en"><head><meta http-equiv="Content-Type" content="text/html; charset=UTF-8">
        <title>yuya</title>
        <meta name="viewport" content="width=device-width, initial-scale=1">
        <meta name="description" content="Penwing's Website">
        <link rel="stylesheet" href="/assets/style.css">
        <link rel="icon" href="/assets/flake.png" type="image/png">
      </head>
      <body>
    <div class="sidenav">
      <h3><a href="/">Summary</a></h3>
      <h3><a href="/tools">Tools</a></h3>
      <a class="description" href="https://www.penwing.org/tools.html#nginx">proxy</a>
      <a class="description" href="https://www.penwing.org/tools.html#cloudflare">domain</a>
      <a class="description" href="https://www.penwing.org/tools.html#portainer">dashboard</a>
      <a class="description" href="https://www.penwing.org/tools.html#pihole">adblocker</a>
      <a class="description" href="https://www.penwing.org/tools.html#searxng">search</a>
      <a class="description" href="https://www.penwing.org/tools.html#forgejo">git server</a>
      <a class="description" href="https://www.penwing.org/tools.html#jellyfin">streaming</a>
      <a class="description" href="https://www.penwing.org/tools.html#stirling">pdf edit</a>
      <a class="description" href="https://www.penwing.org/tools.html#seafile">storage</a>
      
      <h3><a href="https://www.penwing.org/art.html">my Art</a></h3>
      <h3><a href="https://www.penwing.org/games.html">my Games</a></h3>
    </div>
    <p class="topbar">
    <a href="/">Home</a> - 
    <a href="/tools">Tools</a> - 
    <a href="/art">Art</a> -
    <a href="/games">Games</a>
    </p>
    <div class="main">
    #CONTENT#
    <hr>
    <p id="spacer"><br></p>
    
    <h2 id="epilogue">Epilogue</h2>
    
    <p>Inspired by the geniuses behind <a href="https://perfectmotherfuckingwebsite.com/">perfectwebsite.com</a>, 
    because a webpage does not have to be heavier than <a href="https://github.com/chrislgarry/Apollo-11">the code that took us to the moon</a>.</p>
    
    <p>This page is licensed under <a href="https://creativecommons.org/publicdomain/zero/1.0/">CC0</a></p>
    
    </div>
    </body></html>
    "#;

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


