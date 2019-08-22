extern crate simple_server;

use simple_server::Server;
use motivations::MOTIVATIONS;
use pick_one::pick_one_str;
use handlebars::Handlebars;
use std::collections::BTreeMap;

fn main() {
    const template: &str = r#"<!DOCTYPE html>
        <html>
        <head>
            <title>RustTogether</title>
            <link rel="stylesheet" href="styles.css"/>
            <style>
            body {
                background: url('img/crab-{{ num }}.jpg'); 
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
        let mut new_template = template.to_string();
        let mut handlebars = Handlebars::new();
        let crab_nums = vec!["1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13"];
        let mut data = BTreeMap::new();
        data.insert("num", pick_one_str(&crab_nums[..]));
        data.insert("motivation", pick_one_str(&MOTIVATIONS));
        Ok(response.body(handlebars.render_template(&new_template, &data)
            .expect("expect String")
            .as_bytes()
            .to_vec())?)
    });

    

    server.listen("localhost", "4000");

    
}
