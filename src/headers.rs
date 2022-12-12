use std::error;

use reqwest::header;

use crate::config::Configuration;

pub fn get_headers() -> Result<header::HeaderMap, Box<dyn error::Error>> {
    let config = Configuration::new();
    let mut headers = header::HeaderMap::new();
    headers.insert("authority", config.authority.parse()?);
    headers.insert("accept", "text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.9".parse()?);
    headers.insert(
        "accept-language",
        "zh-CN,zh;q=0.9,en;q=0.8,en-GB;q=0.7,en-US;q=0.6".parse()?,
    );
    headers.insert("cache-control", "max-age=0".parse()?);
    headers.insert(header::COOKIE, config.cookie.parse()?);
    headers.insert(
        "sec-ch-ua",
        "\"Microsoft Edge\";v=\"105\", \" Not;A Brand\";v=\"99\", \"Chromium\";v=\"105\""
            .parse()?,
    );
    headers.insert("sec-ch-ua-mobile", "?0".parse()?);
    headers.insert("sec-ch-ua-platform", "\"Windows\"".parse()?);
    headers.insert("sec-fetch-dest", "document".parse()?);
    headers.insert("sec-fetch-mode", "navigate".parse()?);
    headers.insert("sec-fetch-site", "same-origin".parse()?);
    headers.insert("sec-fetch-user", "?1".parse()?);
    headers.insert("upgrade-insecure-requests", "1".parse()?);
    headers.insert("user-agent", config.user_agent.parse()?);

    Ok(headers)
}
