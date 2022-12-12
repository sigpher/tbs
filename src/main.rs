use jsonpath_lib as jsonpath;
use regex::Regex;
use std::error;
use std::fs;
use std::io::Write;
use tbs::config::Configuration;
use tbs::headers::get_headers;

#[tokio::main]
async fn main() -> Result<(), Box<dyn error::Error>> {
    let mut csv_file = fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("result.csv")?;
    // csv_file.write_all(b"pic_url,item_loc,view_sales\n")?;
    let config = Configuration::new();
    for value in config.auctions.iter() {
        csv_file.write_all((value.to_string() + ",").as_bytes())?;
    }
    csv_file.write_all(b"\n")?;

    for index in 0..config.pages {
        let url = format!("https://s.taobao.com/search?q={}&imgfile=&js=1&stats_click=search_radio_all%3A1&initiative_id=staobaoz_20220927&ie=utf8&bcoffset=1&ntoffset=1&p4ppushleft=2%2C48&s={}",config.keyword,44*index);
        let client = reqwest::Client::new();
        let headers = get_headers()?;
        let resp = client.get(url).headers(headers).send().await?;
        let text = resp.text().await?;
        let re = Regex::new(r"g_page_config = (?P<json_text>.*?}});")?;
        let mut json_text = String::from("");
        for cap in re.captures_iter(&text) {
            json_text = cap["json_text"].to_string();
        }
        let json_body = serde_json::from_str(&json_text)?;
        let mut selector = jsonpath::selector(&json_body);
        let auctions = selector("$.mods.itemlist.data.auctions[*]")?;

        for auction in auctions {
            for value in config.auctions.iter() {
                csv_file.write_all(b"_,")?;
            }
            csv_file.write_all(b"\n")?;
        }
    }

    Ok(())
}
