use ratatui::Frame;
use ratatui::layout::{Constraint, Rect};
use ratatui::prelude::{Color, Style};
use ratatui::widgets::{HighlightSpacing, Row, Table, TableState};
use crate::netease::login::Login;
pub const MENU_LIST: [&str; 10] = [
    "0. 每日推荐歌曲",
    "1. 每日推荐歌单",
    "2. 我的歌单",
    "3. 我的收藏",
    "4. 私人FM",
    "5. 专辑列表",
    "6. 搜索",
    "7. 排行榜",
    "8. 精选歌单",
    "9. 热门歌手",
];

/** # 梅须逊雪三分白，雪却输梅一段香
```
---梅花也许比不上雪花的晶莹洁白，雪花却输给梅花的清香芬芳。
```
*/
pub struct Menu {
    state_l: TableState,
    state_r: TableState,
    items: [&'static str; 10],
    select: usize,
    login: Login,
}

impl Menu {
    pub fn new() -> Menu {
        Menu {
            state_l: TableState::default(),
            state_r: TableState::default(),
            items: MENU_LIST,
            select: 0,
            login: Login::default(),
        }
    }
    pub fn draw(&mut self, frame: &mut Frame, a: Rect, b: Rect) {
        let (l, r) = table_split(self.items.to_vec());
        let (mut l,mut r) = (table_items(l), table_items(r));
        if self.state(self.select) {
            l = l.highlight_symbol("   ").highlight_style(Style::default().fg(Color::White));
        } else {
            r = r.highlight_symbol("   ").highlight_style(Style::default().fg(Color::White));
        }
        frame.render_stateful_widget(l, a, &mut self.state_l);
        frame.render_stateful_widget(r, b, &mut self.state_r);
    }
    fn selected(&mut self) {
        match self.select {
            0 => {},
            1 => {},
            2 => {},
            3 => {},
            4 => {},
            5 => {},
            _ => {},
        }
    }
    fn state(&mut self, select: usize) -> bool {
        if select < self.items.len() / 2 {
            // self.unselect_r();
            self.state_l.select(Some(select));
            false
        } else {
            // self.unselect_l();
            self.state_r.select(Some(select - self.items.len() / 2));
            true
        }
    }
    pub fn previous(&mut self) {
        if self.select == 0 {
            self.select = self.items.len() - 1;
        } else {
            self.select = self.select - 1;
        }

    }
    pub fn next(&mut self) {
        if self.select == self.items.len() - 1 {
            self.select = 0;
        } else {
            self.select = self.select + 1;
        }
    }
    pub fn left(&mut self) {
        if self.select >= self.items.len() / 2 {
            self.select -= self.items.len() / 2;
        } else if self.select == 0 {
            self.select = self.items.len() - 1;
        } else {
            self.select += self.items.len() / 2 - 1;
        }
    }
    pub fn right(&mut self) {
        if self.select < self.items.len() / 2 {
            self.select += self.items.len() / 2;
        } else if self.select == self.items.len() - 1 {
            self.select = 0;
        } else {
            self.select -= self.items.len() / 2 - 1;
        }
    }
    pub fn _unselect_l(&mut self) {
        self.state_l.select(None);
    }
    pub fn _unselect_r(&mut self) {
        self.state_r.select(None);
    }
    pub fn _unselect(&mut self) {
        self.select = 0;
        self.state_l.select(None);
        self.state_r.select(None);
    }

}
fn table_split(i: Vec<&str>) -> (Vec<&str>, Vec<&str>) {
    let l = i.len();
    let m = l / 2;
    (i[0..m].to_vec(), i[m..l].to_vec())
}
fn table_items(i: Vec<&str>) -> Table {
    let mut rows = Vec::new();
    for o in i {
        rows.push(Row::new(vec![o]));
    }
    let widths = [Constraint::Length(25)];
    Table::new(rows, widths)
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().fg(Color::LightRed))
        .highlight_symbol("⇒  ")
        .highlight_spacing(HighlightSpacing::Always)
}