use regex::Regex;
use scraper::{Html, Selector};
use std::env;
use std::error::Error;
use std::ffi::OsStr;
use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use tokio::task::JoinSet;
// https://voz.vn/t/viet-nam-san-sang-trao-%C4%91oi-voi-my-%C4%91e-%C4%91ua-muc-thue-nhap-khau-ve-0.1087123/page-2

fn get_url_voz(url: &str, num: i32) -> String {
    let re = Regex::new(r"(https://voz\.vn/t/.*/page-)\d+").unwrap();
    re.replace(url, |caps: &regex::Captures| format!("{}{}", &caps[1], num))
        .into_owned()
}
