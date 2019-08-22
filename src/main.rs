extern crate simple_server;

use simple_server::Server;

fn main() {
    const template: &str = r#"<!DOCTYPE html>
        <html>
        <head>
            <title>RustTogether</title>
            <link rel="stylesheet" href="styles.css"/>
            <style>
            body {
                background: url('img/crab-13.jpg'); 
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
                hey girl
                </div>
            </h1>
            </div>
        </body>
        </html>"#;
    let server = Server::new(|request, mut response| {
        Ok(response.body(template.as_bytes().to_vec())?)
    });

    

    server.listen("localhost", "4000");

    
}
