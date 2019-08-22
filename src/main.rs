extern crate simple_server;

use simple_server::Server;
use motivations::MOTIVATIONS;
use pick_one::pick_one_str;

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
        let mut crab_nums = vec![];
        for i in 1..13 {
            crab_nums.push(i.to_string());
        }
        let mut new_crab_nums = vec![];
        for crab_str_num in crab_nums.iter() {
            new_crab_nums.push(&crab_str_num[..]);
        }
       
        new_template = new_template.replace("{{ num }}", pick_one_str(&new_crab_nums[..]));
        new_template = new_template.replace("{{ motivation }}", pick_one_str(&MOTIVATIONS));
        Ok(response.body(new_template.as_bytes().to_vec())?)
    });

    

    server.listen("localhost", "4000");

    
}
