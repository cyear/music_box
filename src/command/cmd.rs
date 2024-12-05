use std::env;
use crate::http::download;
use crate::music::music_mp3::Player;

pub async fn init() -> Player {
    let args: Vec<String> = env::args().collect();
    let mut p = Player::new();
    // println!("{:?}", &args);
    if 1 < args.len() && args.len() < 4 {
        if args[1] == "url" {
            // println!("URL: {}", args[2]);
            if let Ok(b) = download::get(&args[2]).await {
                // println!("下载成功！长度：{}", b.len());
                p.add_bytes(b);
            }
        } else if args[1] == "file" {
            // println!("文件: {}", args[2]);
            p.add(&args[2]);
        }
    }
    p
}