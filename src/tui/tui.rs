/*!负责UI实现逻辑*/

use std::fmt::{Debug, Formatter};
use std::time::Duration;
use crossterm::event::{self, Event, KeyCode, KeyEventKind, poll};
use crossterm::{
    terminal::{
        disable_raw_mode, enable_raw_mode, EnterAlternateScreen,
        LeaveAlternateScreen,
    },
    ExecutableCommand,
};
use ratatui::{prelude::*};
use std::io::{stdin, stdout, Stdout};
use ratatui::widgets::{Block, Borders};
use crate::netease::login::Login;
use crate::tui::ui::{layout_aabb, bottom_bar, frame_box};
use crate::tui::menu::Menu;

/**
# 面包说用它就对了！

## 用法

```
// new() -> UI
let mut u = UI::new();
loop {
    u.draw();
}
UI::close();
```
 */
pub struct UI {
    terminal: Terminal<CrosstermBackend<Stdout>>,
    menu: Menu,
    login: Login,
}
impl Debug for UI {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Ok(write!(f, "UserName: \n\t{}\nLoginCode: \n\t{}\nLoginUid: \n\t{}", self.login.name, self.login.info.code, self.login.info.uid)?)
    }
}
impl UI {
    pub async fn new() -> UI {
        stdout().execute(EnterAlternateScreen).unwrap();
        enable_raw_mode().unwrap();
        let mut terminal = Terminal::new(CrosstermBackend::new(stdout())).unwrap();
        terminal.clear().unwrap();
        UI {
            terminal,
            menu: Menu::new(),
            login: Login::default(),
        }
    }
    pub async fn login(&mut self) {
        if Login::login_status().await {
            self.terminal.draw(|f| {
                f.render_widget(
                    Block::default()
                        .borders(Borders::NONE)
                        .title_top("账号/密码(空格分割): "),
                    crate::netease::login::centered_rect(f.size(), 35, 35),
                );
            }).unwrap();
            self.keys_events().await.unwrap();
            let mut s = String::new();
            stdin().read_line(&mut s).unwrap();
            let (user, passwd) = s.split_once(" ").expect("输入格式错误");
            // println!("{} {}", user, passwd);
            let login = Login::new(user, passwd).await;
        } else {
            let login = Login::new("", "").await;
            login.refresh();
            println!("刷新成功");
        }
    }
    pub fn close() {
        stdout().execute(LeaveAlternateScreen).unwrap();
        disable_raw_mode().unwrap();
    }
    pub fn exit() {
        UI::close();
        std::process::exit(0);
    }
    pub async fn draw(&mut self) {
        self.terminal.draw(|frame| {
            let (a, b, _c, _d, e) = layout_aabb(frame);
            frame_box(frame);
            bottom_bar(frame, e, &("网易云音乐 [".to_string() + &self.login.name + &String::from("]")));
            self.menu.draw(frame, a, b);
        }).unwrap();
        self.keys_events().await.expect("不支持快捷键！");
    }
    async fn keys_events(&mut self) -> std::io::Result<()> {
        enable_raw_mode().unwrap();
        if poll(Duration::from_millis(500))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') => {
                            UI::exit();
                        }
                        KeyCode::Char('d') => {
                            println!("{:#?}", self);
                        }
                        KeyCode::Up => { self.menu.previous() }
                        KeyCode::Down => { self.menu.next() }
                        KeyCode::Left => { self.menu.left() }
                        KeyCode::Right => { self.menu.right() }
                        _ => {}
                    }
                }
            } else {}
        }
        disable_raw_mode().unwrap();
        Ok(())
    }
}