use ::config::{File, Value};

fn get_config() -> config::Config {
    let settings = config::Config::builder()
        .add_source(File::with_name("settings.toml"))
        .build()
        .unwrap();
    settings
}

#[derive(Debug)]
pub struct Configuration {
    pub keyword: String,
    pub pages: i64,
    pub cookie: String,
    pub user_agent: String,
    pub authority: String,
    pub auctions: Vec<Value>,
}

impl Configuration {
    pub fn new() -> Configuration {
        // pub fn new() {
        let settings = get_config();
        let keyword = settings.get_string("search.keyword").unwrap();
        let pages = settings.get_int("search.pages").unwrap();
        let cookie = settings.get_string("headers.cookie").unwrap();
        let user_agent = settings.get_string("headers.user-agent").unwrap();
        let authority = settings.get_string("headers.authority").unwrap();
        let auctions = settings.get_array("content.auctions").unwrap();
        Configuration {
            keyword,
            pages,
            cookie,
            user_agent,
            authority,
            auctions,
        }
    }
}
