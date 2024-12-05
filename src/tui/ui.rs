/*! 画画画！ */
use ratatui::{Frame};
use ratatui::layout::*;
use ratatui::prelude::*;
use ratatui::widgets::*;
use ratatui::widgets::block::Title;

pub fn frame_box<'a>(frame: &mut Frame) -> Block<'a> {
    let m = Block::default()
        .title(
            Title::from(
                " Bread Box Music ".to_string()
            ).alignment(Alignment::Center)
        )
        .title_style(Style::default().fg(Color::LightCyan))
        .borders(Borders::TOP)
        .border_style(
            Style::default()
                .fg(Color::LightRed)
        ).border_type(BorderType::Rounded);

    frame.render_widget(&m, frame.size());
    m
}

/** # 布局
```
(Rect, Rect, Rect, Rect, Rect)
  左菜单 右菜单 歌词 进度条 底部
```
 */
pub fn layout_aabb(frame: &mut Frame) -> (Rect, Rect, Rect, Rect, Rect) {
    let l = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage(10),
                Constraint::Percentage(20),
                Constraint::Percentage(20),
                Constraint::Percentage(40),
                Constraint::Percentage(10),
            ]
                .as_ref(),
        )
        .split(frame.size());
    let m = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(25),
                Constraint::Percentage(25),
                Constraint::Percentage(25),
                Constraint::Percentage(25),
            ]
                .as_ref(),
        )
        .split(l[2]);
    (m[1], m[2], l[3], l[4], l[1])
}

pub fn bottom_bar(frame: &mut Frame, l: Rect, text: &str) {
    let mut l = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(25),
                Constraint::Percentage(75),
            ]
                .as_ref(),
        )
        .split(l);
    l = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage(70),
                Constraint::Percentage(30),
            ]
                .as_ref(),
        )
        .split(l[1]);
    frame.render_widget(
        Block::default()
            .borders(Borders::NONE)
            .title_bottom(
                Line::from(text)
                    .left_aligned()
            )
            .title_style(
                Style::default()
                    .fg(Color::LightGreen)
            ),
        l[0]
    );
}