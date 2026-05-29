use std::time::{Duration, Instant};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Animal {
    TortueDeMer,
    Panda,
    Lapin,
    Guepard,
    Faucon,
    Eclair,
    Legende,
}

impl Animal {
    pub fn from_cps(cps: f64) -> Self {
        match cps {
            c if c >= 13.0 => Self::Legende,
            c if c >= 11.0 => Self::Eclair,
            c if c >= 9.0 => Self::Faucon,
            c if c >= 7.0 => Self::Guepard,
            c if c >= 5.0 => Self::Lapin,
            c if c >= 3.0 => Self::Panda,
            _ => Self::TortueDeMer,
        }
    }

    pub fn name(&self) -> &str {
        match self {
            Self::TortueDeMer => "Tortue de mer",
            Self::Panda => "Panda",
            Self::Lapin => "Lapin",
            Self::Guepard => "Guepard",
            Self::Faucon => "Faucon",
            Self::Eclair => "Eclair",
            Self::Legende => "Legende",
        }
    }

    pub fn speed_kmh(&self) -> f64 {
        match self {
            Self::TortueDeMer => 22.0,
            Self::Panda => 32.0,
            Self::Lapin => 45.0,
            Self::Guepard => 58.0,
            Self::Faucon => 72.0,
            Self::Eclair => 88.0,
            Self::Legende => 100.0,
        }
    }

    pub fn emoji(&self) -> &str {
        match self {
            Self::TortueDeMer => "\u{1f422}",
            Self::Panda => "\u{1f43c}",
            Self::Lapin => "\u{1f430}",
            Self::Guepard => "\u{1f406}",
            Self::Faucon => "\u{1f985}",
            Self::Eclair => "\u{26a1}",
            Self::Legende => "\u{1f451}",
        }
    }
}

pub struct TestEngine {
    clicks: u64,
    start_time: Option<Instant>,
    finish_time: Option<Instant>,
    duration: Duration,
}

impl TestEngine {
    pub fn new() -> Self {
        Self {
            clicks: 0,
            start_time: None,
            finish_time: None,
            duration: Duration::from_secs(10),
        }
    }

    pub fn start(&mut self, duration: Duration) {
        self.clicks = 0;
        self.start_time = Some(Instant::now());
        self.finish_time = None;
        self.duration = duration;
    }

    pub fn click(&mut self) {
        self.clicks += 1;
    }

    pub fn elapsed(&self) -> Duration {
        match self.start_time {
            Some(t) => Instant::now().duration_since(t),
            None => Duration::ZERO,
        }
    }

    pub fn is_finished(&self) -> bool {
        self.finish_time.is_some() || self.elapsed() >= self.duration
    }

    pub fn finish(&mut self) {
        if self.finish_time.is_none() {
            self.finish_time = Some(Instant::now());
        }
    }

    pub fn cps(&self) -> f64 {
        let elapsed = match self.finish_time {
            Some(t) => t.duration_since(self.start_time.unwrap()),
            None => self.elapsed(),
        };
        let secs = elapsed.as_secs_f64();
        if secs > 0.0 {
            self.clicks as f64 / secs
        } else {
            0.0
        }
    }

    pub fn clicks(&self) -> u64 {
        self.clicks
    }

    pub fn animal(&self) -> Animal {
        Animal::from_cps(self.cps())
    }

    pub fn progress(&self) -> f64 {
        let elapsed = self.elapsed().as_secs_f64();
        let total = self.duration.as_secs_f64();
        if total > 0.0 {
            (elapsed / total).min(1.0)
        } else {
            0.0
        }
    }

    pub fn remaining_secs(&self) -> f64 {
        (self.duration.as_secs_f64() - self.elapsed().as_secs_f64()).max(0.0)
    }

    pub fn duration_secs(&self) -> u64 {
        self.duration.as_secs()
    }
}
