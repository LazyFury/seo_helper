use std::time::Duration;

use super::task::Task;

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

    pub fn build(url: String) -> Spider {
        let mut spider = Spider::new(Duration::from_millis(100));
        spider.add_task(Task::new(url));
        spider
    }

    pub fn add_task(&mut self, task: Task) {
        self.tasks.push(task);
    }

    pub fn crawl(&mut self) {
        return self.crawl_self(0);
    }
    fn crawl_self(&mut self, i: usize) {
        let len = self.tasks.len();
        if i >= len {
            return;
        }
        let task = self.tasks.get_mut(i).unwrap();

        println!(
            "request on :{}  【index】:{} 【count】: {:?}",
            task.url, i, len
        );
        let urls = task.crawl(&self.client).expect("执行任务出错");
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

    pub fn has_task(&self, url: &str) -> bool {
        for task in &self.tasks {
            if task.url == url {
                return true;
            }
        }
        false
    }
}
