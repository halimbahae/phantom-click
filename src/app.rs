use std::time::{Duration, Instant};

use crossterm::event::{KeyCode, KeyEvent};
use chrono::Local;

use crate::storage::{ScoreEntry, Scores};
use crate::test::TestEngine;

pub const DURATIONS: [u64; 10] = [1, 3, 5, 10, 15, 30, 60, 100, 180, 900];
pub const DURATION_LABELS: [&str; 10] = ["1s", "3s", "5s", "10s", "15s", "30s", "60s", "100s", "180s", "900s"];

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AppState {
    Menu,
    Ready,
    Countdown,
    Running,
    Result,
}

pub struct App {
    pub state: AppState,
    pub selected_idx: usize,
    pub engine: TestEngine,
    pub scores: Scores,
    pub countdown_start: Instant,
    pub matrix_frame: u64,
    pub should_quit: bool,
}

impl App {
    pub fn new() -> Self {
        Self {
            state: AppState::Menu,
            selected_idx: 3,
            engine: TestEngine::new(),
            scores: Scores::load(),
            countdown_start: Instant::now(),
            matrix_frame: 0,
            should_quit: false,
        }
    }

    pub fn handle_key(&mut self, key: KeyEvent) {
        match self.state {
            AppState::Menu => self.handle_menu_key(key),
            AppState::Ready => self.handle_ready_key(key),
            AppState::Countdown => {}
            AppState::Running => self.handle_running_key(key),
            AppState::Result => self.handle_result_key(key),
        }
    }

    fn handle_menu_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Up | KeyCode::Char('k') => {
                self.selected_idx = self.selected_idx.saturating_sub(5);
            }
            KeyCode::Down | KeyCode::Char('j') => {
                self.selected_idx = (self.selected_idx + 5).min(DURATIONS.len() - 1);
            }
            KeyCode::Left | KeyCode::Char('h') => {
                self.selected_idx = self.selected_idx.saturating_sub(1);
            }
            KeyCode::Right | KeyCode::Char('l') => {
                self.selected_idx = (self.selected_idx + 1).min(DURATIONS.len() - 1);
            }
            KeyCode::Enter => {
                self.state = AppState::Ready;
            }
            KeyCode::Char('q') | KeyCode::Char('Q') => {
                self.should_quit = true;
            }
            KeyCode::Char(c) if c.is_ascii_digit() => {
                let digit = c.to_digit(10).unwrap() as usize;
                self.selected_idx = if digit == 0 { 9 } else { digit - 1 };
                self.state = AppState::Ready;
            }
            _ => {}
        }
    }

    fn handle_ready_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Enter | KeyCode::Char(' ') => {
                self.countdown_start = Instant::now();
                self.state = AppState::Countdown;
            }
            KeyCode::Char('r') | KeyCode::Char('R') | KeyCode::Esc => {
                self.state = AppState::Menu;
            }
            KeyCode::Char('q') | KeyCode::Char('Q') => {
                self.should_quit = true;
            }
            _ => {}
        }
    }

    fn handle_running_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char(' ') => {
                self.engine.click();
            }
            KeyCode::Char('q') | KeyCode::Char('Q') => {
                self.should_quit = true;
            }
            _ => {}
        }
    }

    fn handle_result_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char('r') | KeyCode::Char('R') => {
                self.state = AppState::Menu;
                self.engine = TestEngine::new();
            }
            KeyCode::Char('q') | KeyCode::Char('Q') => {
                self.should_quit = true;
            }
            _ => {}
        }
    }

    pub fn update(&mut self) {
        self.matrix_frame += 1;

        match self.state {
            AppState::Countdown => {
                if self.countdown_start.elapsed() >= Duration::from_secs(3) {
                    let secs = DURATIONS[self.selected_idx];
                    self.engine.start(Duration::from_secs(secs));
                    self.state = AppState::Running;
                }
            }
            AppState::Running => {
                if self.engine.is_finished() {
                    self.engine.finish();
                    let entry = ScoreEntry {
                        cps: self.engine.cps(),
                        clicks: self.engine.clicks(),
                        duration_secs: DURATIONS[self.selected_idx],
                        animal: self.engine.animal().name().to_string(),
                        date: Local::now().format("%Y-%m-%d").to_string(),
                    };
                    self.scores.add_score(entry);
                    self.state = AppState::Result;
                }
            }
            _ => {}
        }
    }

    pub fn countdown_display(&self) -> String {
        if self.state != AppState::Countdown {
            return String::new();
        }
        let elapsed = self.countdown_start.elapsed().as_secs_f64();
        let remaining = 3.0 - elapsed;
        if remaining <= 0.0 {
            ">> GO! <<".to_string()
        } else {
            (remaining.ceil() as u8).to_string()
        }
    }
}
