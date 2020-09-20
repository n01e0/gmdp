#[macro_use]
extern crate clap;

use std::fs;
use std::error::Error;
use std::process::exit;
use warp::Filter;
use comrak::{markdown_to_html, ComrakOptions};

#[tokio::main]
async fn main() {
    let matches = clap_app!(gmdp =>
        (version:       crate_version!())
        (author:        crate_authors!())
        (about:         crate_description!())
        (@arg port: -p --port "port(default 6419)")
        (@arg only_parse: -o --only_parse "without browse, only parse markdown")
        (@arg path: +required "markdown file")
    ).get_matches();

    let path = matches.value_of("path").unwrap() ;
    let md = parse_markdown(path).unwrap_or_else(|e| { eprintln!("{}", e); exit(1);});


    if matches.is_present("only_parse") {
        println!("{}", &md);
    } else {
        let port = matches.value_of("port").unwrap_or("6419").parse::<u16>().unwrap_or_else(|e| { eprintln!("{}", e); exit(1);});
        let md = format!("{}", md);
        let server = warp::any().map(move || {
            warp::reply::html(md.to_owned())
        });
        println!("Server running 127.0.0.1:{}", port);
        warp::serve(server).run(([127,0,0,1], port)).await;
    }
}

fn parse_markdown(path: &str) -> Result<String, Box<dyn Error>> {
    let content = fs::read_to_string(path)?;
    Ok(markdown_to_html(&content, &ComrakOptions::default()))
}
