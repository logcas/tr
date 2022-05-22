use serde::{Deserialize};
use anyhow::{Result, Ok};
use clap::{AppSettings, Parser};
use md5;
use reqwest;
use colored::*;
use dotenv::dotenv;
use std::{env};
use random_number::random;

#[derive(Parser)]
#[clap(author = "lxz", version, about, long_about = None)]
#[clap(args_override_self = true)]
#[clap(allow_negative_numbers = true)]
#[clap(global_setting(AppSettings::DeriveDisplayOrder))]
struct Cli {
    word: Option<String>
}

fn get_sign(appid: &str, word: &str, salt: &str, sk: &str) -> String {
    format!("{}{}{}{}", appid, word, salt, sk)
}

#[derive(Deserialize, Debug)]
struct TranslateResult {
    src: String,
    dst: String
}

#[derive(Deserialize, Debug)]
struct TranslateResponse {
    from: String,
    to: String,
    trans_result: Vec<TranslateResult>
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    let cli = Cli::parse();

    if let Some(t) = cli.word {
        let appid = env::var("APPID").unwrap();
        let sk = env::var("SK").unwrap();
        let random_num: u16 = random!();
        let salt = format!("{}", random_num);
        let sign = format!("{:x}", md5::compute(get_sign(&appid, &t, &salt, &sk)));
        let request_url = format!("http://api.fanyi.baidu.com/api/trans/vip/translate?q={}&from=en&to=zh&appid={}&salt={}&sign={}", &t, &appid, &salt, sign);
        let resp = reqwest::get(request_url).await?.json::<TranslateResponse>().await?;
        println!("源文：{}", &t);
        if resp.trans_result.is_empty() {
            println!("{}", "无法翻译该词语".red());
        } else {
            let r: Vec<String> = resp.trans_result.into_iter().map(|x| x.dst).collect();
            println!("译文：{}", r.join(";").cyan());
        }
        Ok(())
    } else {
        println!("请输入词语");
        Ok(())
    }
    
}
