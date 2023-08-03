use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Copy)]
pub enum Changefreq {
    Always,
    Hourly,
    Daily,
    Weekly,
    Monthly,
}

impl ToString for Changefreq {
    fn to_string(&self) -> String {
        match self {
            Changefreq::Always => "always",
            Changefreq::Hourly => "hourly",
            Changefreq::Daily => "daily",
            Changefreq::Weekly => "weekly",
            Changefreq::Monthly => "monthly",
        }
        .to_string()
    }
}

#[derive(Debug)]
pub struct Url {
    pub loc: String,
    pub lastmod: Option<DateTime<Utc>>,
    pub changefreq: Option<Changefreq>,
    pub priority: Option<f32>,
}

#[derive(Debug)]
pub struct Xml {
    pub urls: Vec<Url>,
    pub version: &'static str,
    pub encoding: &'static str,
    pub urlset_namespace: &'static str,
}

impl ToString for Xml {
    fn to_string(&self) -> String {
        Box::leak(format!(r#"{}{}"#, self.header(), self.urlset()).into_boxed_str()).to_string()
    }
}

impl Xml {
    pub fn new() -> Xml {
        Xml {
            urls: vec![],
            version: "1.0",
            encoding: "UTF-8",
            urlset_namespace: "http://www.sitemaps.org/schemas/sitemap/0.9",
        }
    }

    pub fn add_url(&mut self, url: Url) {
        self.urls.push(url);
    }
    pub fn header(&self) -> &'static str {
        let str = format!(
            "<?xml version=\"{}\" encoding=\"{}\"?>",
            self.version, self.encoding
        );
        Box::leak(str.into_boxed_str())
    }

    pub fn urlset(&self) -> &'static str {
        let mut urls = vec![];
        for url in &self.urls {
            urls.push(self.url(url));
        }
        Box::leak(
            format!(
                "
<urlset xmlns=\"{}\">\n{}\n</urlset>",
                self.urlset_namespace,
                urls.join("\n")
            )
            .into_boxed_str(),
        )
    }

    pub fn url(&self, url: &Url) -> &'static str {
        let lastmod = match url.lastmod {
            Some(last) => last,
            None => Utc::now(),
        };

        let changefreq = match url.changefreq {
            Some(freq) => freq,
            None => Changefreq::Weekly,
        };

        let priority = match url.priority {
            Some(prio) => prio,
            None => 1.0,
        };

        Box::leak(
            format!(
                r#"<url>
    <loc><![CDATA[{}]]></loc>
    <lastmod>{}</lastmod>
    <changefreq>{}</changefreq>
    <priority>{}</priority>
</url>"#,
                url.loc,
                lastmod.to_string(),
                changefreq.to_string(),
                priority
            )
            .into_boxed_str(),
        )
    }
}
