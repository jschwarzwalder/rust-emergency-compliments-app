extern crate simple_server;

use simple_server::Server;
use motivations::MOTIVATIONS;

fn main() {
    const template: &str = r#"<!DOCTYPE html>
        <html>
        <head>
            <title>RustTogether</title>
            <link rel="stylesheet" href="styles.css"/>
            <style>
            body {
                background: url('img/crab-{{image}}.jpg'); 
                background-size: cover;
            }
            </style>
        </head>
        <body>
            <header>
            <span class="logo">RustTogether</span>
            </header>
            <div class="container">
            <h1>
                <div class="motivation">
                {{ motivation }}
                </div>
            </h1>
            </div>
        </body>
        </html>"#;
    let server = Server::new(|request, mut response| {
        let mut new_template = MOTIVATIONS[0];
        Ok(response.body(new_template.as_bytes().to_vec())?)
    });

    

    server.listen("localhost", "4000");

    
}
