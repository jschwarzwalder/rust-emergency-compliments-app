extern crate simple_server;

use simple_server::Server;
use motivations::MOTIVATIONS;
use pick_one::pick_one_str;
use handlebars::Handlebars;
use std::collections::BTreeMap;
use std::env::var;

fn main() {

    let server = Server::new(|request, mut response| {
        
        let mut handlebars = Handlebars::new();
        let template = ("template");
        handlebars.register_template_file(&template, "src/templates/motivation.html" );
        let crab_nums = vec!["1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13"];
        let mut data = BTreeMap::new();
        data.insert("image", pick_one_str(&crab_nums[..]));
        data.insert("motivation", pick_one_str(&MOTIVATIONS));
        Ok(response.body(handlebars.render(&template, &data)
            .expect("expect String")
            .as_bytes()
            .to_vec())?)
    });

    

    server.listen("0.0.0.0", &var("PORT").unwrap());

    
}
