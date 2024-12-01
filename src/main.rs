mod music;
mod http;
mod command;
use command::cmd::args;

#[tokio::main]
async fn main() {
    let mut p = args::init().await;
    p.play();
    loop {
        if p.status().empty {
            break;
        }
        println!("{:?}", p.status());
    }
}