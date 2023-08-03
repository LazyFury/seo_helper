use std::fs::{self};

use clap::{Arg, ArgAction, Command};

fn main() {
    println!("Hello, world!");

    let matches = Command::new("sitemap")
        .about("about")
        .version("1.0")
        .arg_required_else_help(true)
        .args([
            Arg::new("url")
                // .short('u')
                // .long("url")
                .required(true)
                .help("target url")
                .action(ArgAction::Set),
            Arg::new("output")
                .short('o')
                .long("output")
                .action(ArgAction::SetTrue)
                .help("output sitemap.xml"),
        ])
        .get_matches();

    let url = matches.get_one::<String>("url");
    let url = match url {
        Some(url) => url,
        None => {
            println!("url is required");
            return;
        }
    };
    let output = matches.get_one::<bool>("output");
    let output = match output {
        Some(output) => output.to_owned(),
        None => false,
    };
    println!("url: {} output: {}", url, output);
    if !url.is_empty() {
        let mut spider = seo_helper::sitemap::spider::Spider::build(url.to_string());
        let spider = spider.crawl();
        if output {
            let dir = std::env::current_dir().unwrap();
            let cache = dir.join("cache");

            if !cache.exists() {
                let _cache = fs::create_dir(cache.clone());
            }
            let path = cache.join("sitemap.xml");
            print!("{:?}", path);
            spider.gen_xml_from_tasks(path);
        }
    }
}
