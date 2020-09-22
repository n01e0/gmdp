#[macro_use]
extern crate clap;
#[macro_use]
extern crate log;

use std::env;
use std::fs;
use std::io::Write;
use std::process::exit;
use gmdp::daemonize::Daemon;
use std::path::Path;
use warp::Filter;
use comrak::{markdown_to_html, ComrakOptions};

#[tokio::main]
async fn main() {
    let matches = clap_app!(gmdp =>
        (version:       crate_version!())
        (author:        crate_authors!())
        (about:         crate_description!())
        (@arg port: -p --port "port(default 6419)")
        (@arg daemon: -d --daemon "daemonize")
        (@arg log: --log_to "log file path (default /tmp/gmdp.log)")
        (@arg pid_file: --pid_file "pid file (default /tmp/gmdp.pid)")
        (@arg workdir: --work_on "workdir (default /tmp)")
        (@arg only_parse: -o --only_parse "without browse, only parse markdown")
        (@arg path: +required "markdown file")
    ).get_matches();

    if let Err(_) = env::var("RUST_LOG") {
        env::set_var("RUST_LOG", "info");
    }

    env_logger::Builder::from_default_env()
        .format(|buf, record|{
            let ts = buf.timestamp();
            writeln!(
                buf,
                "[{} {} {}] {} {}:{}",
                ts,
                record.level(),
                record.target(),
                record.args(),
                record.file().unwrap_or("unknown"),
                match record.line() {
                    Some(n) => n.to_string(),
                    None => String::from("unknown"),
                }
            )
        }).init();

    if matches.is_present("only_parse") {
        let path = matches.value_of("path").unwrap();
        let md = parse_markdown(path);
        println!("{}", &md);
    } else {
        let port = matches.value_of("port").unwrap_or("6419").parse::<u16>().unwrap_or_else(|e| { error!("{}", e); exit(1);});
        let path = fs::canonicalize(Path::new(matches.value_of("path").unwrap())).unwrap();
    
        if matches.is_present("daemon") {
            let mut daemon = Daemon::new(port);
            daemon.log = matches.value_of("log");
            daemon.pid_file = matches.value_of("pid_file");
            daemon.workdir = matches.value_of("workdir");
            match daemon.daemonize() {
                Ok(_) => (),
                Err(e) => error!("{}", e)
            }
        }

        let server = warp::any().map(move ||{
            let md = parse_markdown(path.to_str().unwrap());
            warp::reply::html(md)
        });

        info!("Server running 127.0.0.1:{}, pid = {}", port, std::process::id());
        warp::serve(server).run(([127, 0, 0, 1], port)).await;
    }
}

fn parse_markdown(path: &str) -> String {
    let content = fs::read_to_string(path).unwrap_or_else(|e|{error!("{}", e);exit(1)});
    markdown_to_html(&content, &ComrakOptions::default())
}
