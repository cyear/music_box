use std::io::{Read, stdin};
use std::time::Duration;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use netease_cloud_music_api::*;
use ratatui::{layout::*, prelude::*};
// use crate::netease::login;

pub struct Login {
    pub name: String,
    pub(crate) info: LoginInfo,
}
impl Login {
    pub async fn new(email: &str, password: &str) -> Login {
        let password = password.trim();
        let mut login = MusicApi::new(32);
        if !(email.is_empty() || password.is_empty()) {
            let l = login.login(email.to_string(), password.to_string()).await;
            println!("{:?}", l);
        }
        // let user: LoginInfo = login.login_status().await.unwrap();
        // Login {
        //     name: user.nickname.clone(),
        //     info: user,
        // }]
        // login.set_proxy("http://127.0.0.1:7890").unwrap();
        // println!("{:?}", login.recommend_resource().await.unwrap());
        let l = login.login_qr_create().await.unwrap();
        // enable_raw_mode().unwrap();
        disable_raw_mode().unwrap();
        println!("{:?}", l);

        loop {
            println!("{:?}", login.login_qr_check(l.1.clone()).await.unwrap());
            let mut s = String::new();
            stdin().read_line(&mut s).unwrap();
            if s.trim() == "q" {
                break;
            }
        }
        enable_raw_mode().unwrap();
        println!("{:?}", login.login_qr_check(l.1.clone()).await.unwrap());
        let l = login.login_status().await.unwrap();
        println!("{:?}", l);

        Login {
            name: l.nickname.clone(),
            info: l.clone(),
        }
    }
    pub fn default() -> Login {
        Login {
            name: String::from("未登录"),
            info: LoginInfo {
                code: 0,
                uid: 0,
                nickname: "".to_string(),
                avatar_url: "".to_string(),
                msg: "".to_string(),
            },
        }
    }
    pub fn refresh(mut self) {
        self.name = self.info.nickname;
    }
    pub async fn login_status() -> bool {
        false
    }
    pub async fn logout() {

    }
}

pub fn centered_rect(r: Rect, percent_x: u16, percent_y: u16) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}


// fn if_path_exists(path: &str) -> bool {
//     use std::path::Path;
//     Path::new(path).exists()
// }

fn json_to_file(json: serde_json::Value, path: &str) {
    use std::fs::File;
    use std::io::Write;
    let mut file = File::create(path).unwrap();
    file.write_all(json.to_string().as_bytes()).unwrap();
}
// fn file_to_json(path: &str) -> Result<Value> {
//     use std::fs::File;
//     use std::io::Read;
//     let mut file = File::open(path).unwrap();
//     let mut data = String::new();
//     file.read_to_string(&mut data).unwrap();
//     serde_json::from_str(&data)
// }