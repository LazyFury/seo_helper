use scraper::Selector;

#[derive(Debug)]
pub enum Progress {
    WAIT = 0,
    REQ = 1,
    DONE = 2,
}

#[derive(Debug)]
pub struct Task {
    pub url: String,
    pub progress: Progress,
}

impl Task {
    pub fn new(url: String) -> Task {
        Task {
            url: url.to_string(),
            progress: Progress::WAIT,
        }
    }

    pub fn crawl(&mut self, c: &reqwest::blocking::Client) -> Result<Vec<String>, String> {
        self.progress = Progress::REQ;
        let str = match c.get(&self.url).send() {
            Ok(r) => match r.text() {
                Ok(s) => s,
                Err(e) => return Err(e.to_string()),
            },
            Err(e) => return Err(format!("请求url出错! {:?}", e.to_string())),
        };

        // println!("get response:{}", str);

        // parse html
        let doc = scraper::Html::parse_document(&str);
        let selector = Selector::parse("a").unwrap();
        let links = doc.select(&selector);
        // urls
        let mut urls = vec![];
        // base url
        let target_url = url::Url::parse(&self.url).expect("url 格式错误");

        for link in links {
            let href = link.value().attr("href");

            let href = match href {
                Some(str) => str,
                None => continue,
            };
            if href.is_empty() {
                continue;
            }
            // println!("get href: {:?}", href);
            let url = target_url.join(href).expect("拼接相对路径url出错");
            let url = url.to_string();
            if Task::valide_url(&self.url, &url) {
                // println!("add url:{}", url);
                urls.push(url.to_string());
            }
        }
        self.progress = Progress::DONE;
        Ok(urls)
    }

    pub fn valide_url(host: &str, url: &str) -> bool {
        let is_http = regex::Regex::new(r#"^https?://"#).unwrap();
        let is_relative = regex::Regex::new(r#"^\/"#).unwrap();
        let is_inside = regex::Regex::new(&host).unwrap();

        // url with http but inside
        if is_http.is_match(&url) && is_inside.is_match(&url) {
            return true;
        }
        // relative_url
        is_relative.is_match(&url)
    }

    pub fn change_progress(&mut self, progress: Progress) {
        self.progress = progress;
    }
}
