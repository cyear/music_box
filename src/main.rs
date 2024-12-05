mod tui;
mod http;
mod music;
mod command;
mod netease;
use command::cmd;
use tui::tui::UI;

use notify_rust::Notification;


#[tokio::main]
async fn main() {
    Notification::new()
        .summary("Firefox News")
        .body("This will almost look like a real firefox notification.")
        .icon("firefox")
        .show().unwrap();
    
    let mut p = cmd::init().await;
    if p.status().len > 0 {
        p.play();
        loop {
            println!("{:?}", p.status());
        } 
    } else { 
        let mut u = UI::new().await;
        u.login().await;
        loop {
            u.draw().await;
        }
    }
    
    
}
