pub mod sitemap;

#[cfg(test)]
mod tests {
    use crate::sitemap::spider::Spider;

    #[test]
    fn it_works() {
        Spider::build(
            "http://nuxt-ssr.web-framework-a8u3.1611131761764854.cn-shenzhen.fc.devsapp.net/"
                .to_string(),
        )
        .crawl()
    }
}