use std::{fs::File, io::Write, time::Duration};

use chrono::Utc;

use super::{
    task::Task,
    xml::{Changefreq, Url, Xml},
};
#[derive(Debug)]
pub struct Spider {
    pub tasks: Vec<Task>,
    client: reqwest::blocking::Client,
    delay: Duration,
}

impl Spider {
    pub fn new(duration: Duration) -> Spider {
        Spider {
            tasks: Vec::new(),
            client: reqwest::blocking::Client::new(),
            delay: duration,
        }
    }

    /**

    # Example
    ```rust
        use seo_helper::sitemap::spider::Spider;
        let spider = Spider::build("https://google.com/".to_string());
       // spider.crawl() //it take too long...
    ```
    */
    pub fn build(url: String) -> Spider {
        let mut spider = Spider::new(Duration::from_millis(100));
        spider.add_task(Task::new(url));
        spider
    }

    ///添加任务
    pub fn add_task(&mut self, task: Task) -> &Spider {
        self.tasks.push(task);
        self
    }

    ///带默认参数的爬虫
    pub fn crawl(&mut self) -> &Spider {
        self.crawl_self(0);
        self
    }

    ///递归爬取 分析url 添加到任务
    fn crawl_self(&mut self, i: usize) {
        let len = self.tasks.len();
        if i >= len {
            self.gen_xml_from_tasks();
            return;
        }
        let task = self.tasks.get_mut(i).unwrap();

        println!(
            "request on :{}  【index】:{} 【count】: {:?}",
            task.url, i, len
        );
        let urls = task.crawl(&self.client).unwrap();
        for url in urls {
            if self.has_task(&url) {
                continue;
            }
            self.add_task(Task::new(url));
        }
        std::thread::sleep(self.delay);
        if i < self.tasks.len() - 1 {
            self.crawl_self(i + 1);
        }
    }

    /// has soma task
    pub fn has_task(&self, url: &str) -> bool {
        for task in &self.tasks {
            if task.url == url {
                return true;
            }
        }
        false
    }

    pub fn gen_xml_from_tasks(&self) {
        if self.tasks.len() <= 0 {
            return;
        }
        let mut xml = Xml::new();
        for task in &self.tasks {
            xml.add_url(Url {
                loc: task.url.clone(),
                lastmod: Some(Utc::now()),
                changefreq: Some(Changefreq::Daily),
                priority: Some(0.3),
            });
        }

        let mut file = File::create("sitemap.xml").unwrap();
        writeln!(file, "{}", xml.to_string()).unwrap();
    }
}
