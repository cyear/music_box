use crossterm::{
    event::{self, poll, Event, KeyCode, KeyEventKind},
};
use std::time::Duration;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use std::fs::File;
use std::io::{BufReader, Cursor};
use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink};
use std::time::{SystemTime, UNIX_EPOCH};
use bytes::Bytes;
use crate::tui::tui::UI;

pub struct Player {
    sink: Sink,
    _stream: OutputStream,
    _stream_handle: OutputStreamHandle,
    create_time: u64,
}

#[derive(Debug)]
pub struct Status {
    pub empty: bool,
    pub is_paused: bool,
    pub volume: f32,
    pub len: usize,
    pub runtime: u64,
}

impl std::fmt::Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{ empty: {}, is_paused: {}, volume: {}, len: {}, runtime: {} }}", self.empty, self.is_paused, self.volume, self.len, self.runtime)
    }
}

impl Player {
    pub fn new() -> Self {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();
        sink.set_volume(0.5);
        let current_time = SystemTime::now();
        let since_epoch = current_time.duration_since(UNIX_EPOCH).expect("Time went backwards");
        let create_time = since_epoch.as_secs();
        Player {
            sink,
            _stream,
            _stream_handle: stream_handle,
            create_time,
        }
    }
    pub fn add(&mut self, path: &str) {
        let file = BufReader::new(File::open(path).unwrap());
        let source = Decoder::new(file).unwrap();
        self.sink.append(source);
    }
    pub fn add_bytes(&mut self, byte: Bytes) {
        self.sink.append(Decoder::new(Cursor::new(byte)).unwrap());
    }
    pub fn play(&mut self) {
        self.sink.play();
    }

    pub fn status(&mut self) -> Status {
        keys_events(&self.sink).expect("不支持快捷键！");
        Status {
            empty: self.sink.empty(),
            is_paused: self.sink.is_paused(),
            volume: self.sink.volume(),
            len: self.sink.len(),
            runtime: SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards").as_secs() - self.create_time,
        }
    }
}

fn keys_events(sink: &rodio::Sink) -> std::io::Result<()> {
    enable_raw_mode().unwrap();
    let v = volume(sink);
    if v < 0.0 {
        sink.set_volume(0.0);
    } else if v > 1.0 {
        sink.set_volume(1.0);
    }
    if poll(Duration::from_millis(500))? {
        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Up | KeyCode::Char('j') => sink.set_volume(v + 0.05),
                    KeyCode::Down | KeyCode::Char('k') => sink.set_volume(v - 0.05),
                    KeyCode::Char(' ') | KeyCode::Char('s') => if sink.is_paused() { sink.play() } else { sink.pause() },
                    KeyCode::Char('q') => {
                        UI::close();
                        std::process::exit(0);
                    }
                    _ => {}
                }
            }
        } else {}
    }
    disable_raw_mode().unwrap();
    Ok(())
}

fn volume(sink: &rodio::Sink) -> f32 {
    sink.volume()
}