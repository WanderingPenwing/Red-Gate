use pulldown_cmark::{Parser, html, Event, Tag, HeadingLevel};
use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use std::fs;
use std::time::Duration;
use serde::Deserialize;
use wake_on_lan;

#[derive(Deserialize)]
struct WakeQuery {
    password: String,
}

fn slugify(text: &str) -> String {
    text.to_lowercase()
        .replace(" ", "-")
        .replace(|c: char| !c.is_alphanumeric() && c != '-', "")
}

fn boiler_on() -> bool {
	let addr = "192.168.1.42".parse().unwrap();
    let data = [];  // ping data
	let timeout = Duration::from_millis(20);
    let result = ping_rs::send_ping(&addr, timeout, &data, None);
    match result {
        Ok(_reply) => return true,
        Err(_e) => return false
    }
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

async fn serve_markdown(file_path: &str, query_success: Option<bool>) -> impl Responder {
    // Read the Markdown file
    let markdown_content = fs::read_to_string(file_path).unwrap_or_else(|_| String::from("Error reading markdown file"));

    // Convert Markdown to HTML
    let mut html_content = markdown_to_html(&markdown_content);

    // Create the common HTML wrapper
    let common_html = fs::read_to_string("pages/common.html").unwrap_or_else(|_| String::from("Error reading html file"));

	if file_path == "pages/summary.md" {
		if let Some(success) = query_success {
			let summary_html : String;
			let success_html : String;
			
			if success {
				summary_html = fs::read_to_string("pages/summary_on.html").unwrap_or_else(|_| String::from("Error reading summary on html file"));
				success_html = "<p>Successfuly started boiler, please wait a few minutes for the startup process to end.</p>".to_string();
			} else {
				summary_html = fs::read_to_string("pages/summary_off.html").unwrap_or_else(|_| String::from("Error reading summary off html file"));
				success_html = "<p>Wrong password, try again.</p>".to_string();
			}
			html_content = html_content.replace("~SUMMARY~", &summary_html).replace("~SUCCESS~", &success_html);
			
		} else {
		
			let summary_html = if boiler_on() {
				fs::read_to_string("pages/summary_on.html").unwrap_or_else(|_| String::from("Error reading summary on html file"))
			} else {
				fs::read_to_string("pages/summary_off.html").unwrap_or_else(|_| String::from("Error reading summary off html file"))
			};
			
			html_content = html_content.replace("~SUMMARY~", &summary_html).replace("~SUCCESS~", "");
		}
	}
    // Replace #CONTENT# placeholder with the actual Markdown HTML content
    let page_html = common_html.replace("#CONTENT#", &html_content);

    HttpResponse::Ok().content_type("text/html").body(page_html)
}


async fn summary() -> impl Responder {
    serve_markdown("pages/summary.md", None).await
}

async fn tools() -> impl Responder {
    serve_markdown("pages/tools.md", None).await
}

async fn games() -> impl Responder {
    serve_markdown("pages/games.md", None).await
}

async fn wake(query: web::Query<WakeQuery>) -> impl Responder {
    let password = &query.password;

    if password == "magic" {
    	let mac_address: [u8; 6] = [0xC0, 0x7C, 0xD1, 0xFB, 0xC9, 0x86];
    	
    	let magic_packet = wake_on_lan::MagicPacket::new(&mac_address);
    	
    	let _ = magic_packet.send();
    	
    	serve_markdown("pages/summary.md", Some(true)).await

    } else {
    	serve_markdown("pages/summary.md", Some(false)).await
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
	println!("yuya started");
    let result = HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(summary))  // Route for root
            .route("/tools", web::get().to(tools)) // Route for tools
            .route("/wake", web::get().to(wake))
            .route("/games", web::get().to(games))
            .service(actix_files::Files::new("/assets", "./assets").show_files_listing())
            .default_service(web::route().to(summary))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await;
    println!("yuya stopped");
    result
}


