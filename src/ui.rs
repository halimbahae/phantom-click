use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{Block, BorderType, Borders, Gauge, Paragraph},
};

use crate::app::{App, AppState, DURATIONS, DURATION_LABELS};

const HACK_GREEN: Color = Color::Rgb(0, 255, 64);
const HACK_DIM: Color = Color::Rgb(0, 180, 40);
const HACK_DARK: Color = Color::Rgb(0, 60, 12);

pub fn render(frame: &mut Frame, app: &App) {
    let area = frame.area();

    let bg = Block::default().style(Style::new().bg(Color::Black));
    frame.render_widget(bg, area);

    match app.state {
        AppState::Menu => render_menu(frame, app, area),
        AppState::Ready => render_ready(frame, app, area),
        AppState::Countdown => render_countdown(frame, app, area),
        AppState::Running => render_running(frame, app, area),
        AppState::Result => render_result(frame, app, area),
    }
}

fn centered_rect(area: Rect, width: u16, height: u16) -> Rect {
    let top = (area.height.saturating_sub(height)) / 2;
    let left = (area.width.saturating_sub(width)) / 2;
    Rect::new(area.x + left, area.y + top, width, height)
}

fn render_menu(frame: &mut Frame, app: &App, area: Rect) {
    // Matrix rain background (simple pattern)
    let rain_text = generate_matrix_rain(app, area);
    let rain_par = Paragraph::new(rain_text)
        .style(Style::new().fg(HACK_DARK))
        .alignment(Alignment::Center);
    frame.render_widget(rain_par, area);

    // Main content box
    let box_area = centered_rect(area, 50, 18);
    let main_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Double)
        .border_style(Style::new().fg(HACK_GREEN))
        .style(Style::new().bg(Color::Black));
    frame.render_widget(main_block, box_area);

    let inner = Rect::new(box_area.x + 1, box_area.y + 1, box_area.width.saturating_sub(2), box_area.height.saturating_sub(2));

    let vert = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(6),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(2),
            Constraint::Length(1),
            Constraint::Length(3),
        ])
        .split(inner);

    // ASCII Title
    let title_lines = vec![
        Line::from(Span::styled("██╗  ██╗ █████╗  ██████╗██╗  ██╗", Style::new().fg(HACK_GREEN).bold())),
        Line::from(Span::styled("██║  ██║██╔══██╗██╔════╝██║ ██╔╝", Style::new().fg(HACK_GREEN).bold())),
        Line::from(Span::styled("███████║███████║██║     █████╔╝ ", Style::new().fg(HACK_GREEN).bold())),
        Line::from(Span::styled("██╔══██║██╔══██║██║     ██╔═██╗ ", Style::new().fg(HACK_GREEN).bold())),
        Line::from(Span::styled("██║  ██║██║  ██║╚██████╗██║  ██╗", Style::new().fg(HACK_GREEN).bold())),
        Line::from(Span::styled("╚═╝  ╚═╝╚═╝  ╚═╝ ╚═════╝╚═╝  ╚═╝", Style::new().fg(HACK_GREEN).bold())),
    ];
    let title = Paragraph::new(Text::from(title_lines)).alignment(Alignment::Center);
    frame.render_widget(title, vert[0]);

    // Subtitle
    let subtitle = Paragraph::new(Line::from(Span::styled(
        ">>> TERMINAL CPS HACK SUITE v1.0 <<<",
        Style::new().fg(HACK_DIM).bold(),
    )))
    .alignment(Alignment::Center);
    frame.render_widget(subtitle, vert[1]);

    // Divider
    let divider = Paragraph::new(Line::from(Span::styled(
        "═══════════════════════════════════",
        Style::new().fg(HACK_GREEN),
    )))
    .alignment(Alignment::Center);
    frame.render_widget(divider, vert[2]);

    // "Select Duration" header
    let select_hdr = Paragraph::new(Line::from(Span::styled(
        " SELECT DURATION ",
        Style::new().fg(HACK_GREEN).bold(),
    )))
    .alignment(Alignment::Center);
    frame.render_widget(select_hdr, vert[3]);

    // Duration grid: 2 rows x 5 cols
    let grid_area = vert[4];
    let mid = grid_area.height / 2;
    let row1 = Rect::new(grid_area.x, grid_area.y, grid_area.width, mid);
    let row2 = Rect::new(grid_area.x, grid_area.y + mid, grid_area.width, grid_area.height - mid);

    render_duration_row(frame, app, &DURATIONS[..5], &DURATION_LABELS[..5], 0, row1);
    render_duration_row(frame, app, &DURATIONS[5..], &DURATION_LABELS[5..], 5, row2);

    // Controls header
    let ctrl_hdr = Paragraph::new(Line::from(Span::styled(
        " CONTROLS ",
        Style::new().fg(HACK_GREEN).bold(),
    )))
    .alignment(Alignment::Center);
    frame.render_widget(ctrl_hdr, vert[5]);

    // Controls
    let ctrl_text = Text::from(vec![
        Line::from(Span::styled("Up/Down/Left/Right : Navigate", Style::new().fg(HACK_DIM))),
        Line::from(Span::styled("Enter              : Select & Start", Style::new().fg(HACK_DIM))),
        Line::from(Span::styled("1-9 / 0            : Quick select", Style::new().fg(HACK_DIM))),
    ]);
    let ctrl = Paragraph::new(ctrl_text).alignment(Alignment::Center);
    frame.render_widget(ctrl, vert[6]);
}

fn render_duration_row(
    frame: &mut Frame,
    app: &App,
    durations: &[u64],
    labels: &[&str],
    start_offset: usize,
    area: Rect,
) {
    let n = durations.len();
    if n == 0 {
        return;
    }
    let pct = 100 / n as u16;
    let constraints: Vec<Constraint> = (0..n).map(|_| Constraint::Percentage(pct)).collect();
    let cols = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(constraints)
        .split(area);

    for (global_idx, local_idx) in (start_offset..).zip(0..n) {
        let is_sel = app.selected_idx == global_idx;
        let label = labels[local_idx];
        let secs = durations[local_idx];

        let style = if is_sel {
            Style::new().fg(Color::Black).bg(HACK_GREEN).bold()
        } else {
            Style::new().fg(HACK_DIM)
        };

        let text = format!("[{}] {:>3}s", label, secs);
        let cell = Paragraph::new(Line::from(Span::styled(text, style)))
            .alignment(Alignment::Center);
        frame.render_widget(cell, cols[local_idx]);
    }
}

fn render_ready(frame: &mut Frame, app: &App, area: Rect) {
    let box_area = centered_rect(area, 44, 7);
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Double)
        .border_style(Style::new().fg(HACK_GREEN))
        .title(" READY ")
        .title_alignment(Alignment::Center)
        .style(Style::new().bg(Color::Black));
    frame.render_widget(block, box_area);

    let inner = Rect::new(box_area.x + 1, box_area.y + 1, box_area.width.saturating_sub(2), box_area.height.saturating_sub(2));
    let label = DURATION_LABELS[app.selected_idx];
    let secs = DURATIONS[app.selected_idx];

    let text = Text::from(vec![
        Line::from(""),
        Line::from(Span::styled(
            format!("Duration: {}  ({} seconds)", label, secs),
            Style::new().fg(HACK_GREEN).bold(),
        )),
        Line::from(""),
        Line::from(Span::styled(
            "  Press ENTER to start  ",
            Style::new().fg(HACK_DIM),
        )),
        Line::from(Span::styled(
            "  Press R to go back    ",
            Style::new().fg(HACK_DIM),
        )),
    ]);
    let par = Paragraph::new(text).alignment(Alignment::Center);
    frame.render_widget(par, inner);
}

fn render_countdown(frame: &mut Frame, app: &App, area: Rect) {
    let display = app.countdown_display();
    let is_go = display == ">> GO! <<";

    let style = if is_go {
        Style::new().fg(HACK_GREEN).bold().bg(Color::Black)
    } else {
        Style::new().fg(HACK_GREEN).bold().bg(Color::Black)
    };

    let text = Text::from(vec![
        Line::from(""),
        Line::from(""),
        Line::from(""),
        Line::from(""),
        Line::from(Span::styled(format!("\n\n\n\n       {}", display), style)),
    ]);
    let par = Paragraph::new(text).alignment(Alignment::Center);
    frame.render_widget(par, area);
}

fn render_running(frame: &mut Frame, app: &App, area: Rect) {
    let cps = app.engine.cps();
    let clicks = app.engine.clicks();
    let progress = app.engine.progress();
    let remaining = app.engine.remaining_secs();
    let total = app.engine.duration_secs();
    let animal = app.engine.animal();

    let vert = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(2),
            Constraint::Length(3),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(3),
            Constraint::Length(2),
            Constraint::Length(2),
        ])
        .split(area);

    // CPS big display
    let cps_text = Paragraph::new(Text::from(vec![
        Line::from(Span::styled(
            format!("CPS: {:.1}", cps),
            Style::new().fg(HACK_GREEN).bold().bg(Color::Black),
        )),
    ]))
    .alignment(Alignment::Center);
    frame.render_widget(cps_text, vert[0]);

    // Clicks
    let click_text = Paragraph::new(Line::from(Span::styled(
        format!("Clicks: {}", clicks),
        Style::new().fg(HACK_DIM),
    )))
    .alignment(Alignment::Center);
    frame.render_widget(click_text, vert[1]);

    // Gauge
    let gauge = Gauge::default()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::new().fg(HACK_GREEN)),
        )
        .gauge_style(Style::new().fg(HACK_GREEN).bg(HACK_DARK))
        .label(format!("{:.0}%", progress * 100.0))
        .ratio(progress);
    frame.render_widget(gauge, vert[3]);

    // Time
    let time_text = Paragraph::new(Line::from(Span::styled(
        format!("{:.1}s / {}s", remaining, total),
        Style::new().fg(HACK_DIM),
    )))
    .alignment(Alignment::Center);
    frame.render_widget(time_text, vert[4]);

    // Hint
    let hint = Paragraph::new(Line::from(Span::styled(
        format!("SPACE to click  |  {} {}", animal.emoji(), animal.name()),
        Style::new().fg(HACK_DIM),
    )))
    .alignment(Alignment::Center);
    frame.render_widget(hint, vert[6]);
}

fn render_result(frame: &mut Frame, app: &App, area: Rect) {
    let cps = app.engine.cps();
    let clicks = app.engine.clicks();
    let animal = app.engine.animal();
    let duration = app.engine.duration_secs();

    let vert = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(2),
            Constraint::Length(8),
            Constraint::Length(1),
            Constraint::Length(2),
            Constraint::Min(3),
            Constraint::Length(2),
        ])
        .split(area);

    // Title
    let title = Paragraph::new(Line::from(Span::styled(
        "═══ RESULTS ═══",
        Style::new().fg(HACK_GREEN).bold(),
    )))
    .alignment(Alignment::Center);
    frame.render_widget(title, vert[0]);

    // Card
    let card_text = Text::from(vec![
        Line::from(Span::styled(
            format!("  CPS: {:.1}", cps),
            Style::new().fg(HACK_GREEN).bold(),
        )),
        Line::from(Span::styled(
            format!("  {}  {}", animal.emoji(), animal.name()),
            Style::new().fg(HACK_GREEN),
        )),
        Line::from(Span::styled(
            format!("  Speed: {:.1} km/h", animal.speed_kmh()),
            Style::new().fg(HACK_DIM),
        )),
        Line::from(Span::styled(
            format!("  Clicks: {}   Duration: {}s", clicks, duration),
            Style::new().fg(HACK_DIM),
        )),
    ]);
    let card = Paragraph::new(card_text)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Double)
                .border_style(Style::new().fg(HACK_GREEN))
                .style(Style::new().bg(Color::Black)),
        )
        .alignment(Alignment::Center);
    frame.render_widget(card, vert[1]);

    // Best scores header
    let best = app.scores.best_for(duration);
    let mut score_lines: Vec<Line> = vec![Line::from(Span::styled(
        format!(" Best Scores ({}s)", duration),
        Style::new().fg(HACK_GREEN).bold(),
    ))];

    if best.is_empty() {
        score_lines.push(Line::from(Span::styled(
            " No scores yet!",
            Style::new().fg(HACK_DIM),
        )));
    } else {
        for (i, s) in best.iter().enumerate() {
            score_lines.push(Line::from(Span::styled(
                format!(" {}.  {:.1} CPS  {}  {}", i + 1, s.cps, s.animal, s.clicks),
                Style::new().fg(HACK_DIM),
            )));
        }
    }
    let scores_par = Paragraph::new(Text::from(score_lines))
        .alignment(Alignment::Center)
        .style(Style::new().bg(Color::Black));
    frame.render_widget(scores_par, vert[4]);

    // Controls
    let ctrl = Paragraph::new(Line::from(Span::styled(
        " R  Retry     Q  Quit ",
        Style::new().fg(HACK_DIM),
    )))
    .alignment(Alignment::Center);
    frame.render_widget(ctrl, vert[5]);
}

fn generate_matrix_rain(app: &App, area: Rect) -> Text<'static> {
    let mut lines = Vec::new();
    let frame_offset = app.matrix_frame;
    let chars: Vec<char> = "01アイウエオカキクケコサシスセソタチツテトナニヌネノハヒフヘホマミムメモヤユヨラリルレロワヲンABCDEF".chars().collect();
    let n = chars.len();

    for row in 0..area.height {
        let mut spans = Vec::new();
        for col in 0..area.width {
            let idx = (row as usize * 17 + col as usize * 31 + frame_offset as usize) % n;
            let c = chars[idx];
            let is_bright = (row as usize * 3 + col as usize + frame_offset as usize) % 7 == 0;
            let style = if is_bright {
                Style::new().fg(HACK_GREEN)
            } else {
                Style::new().fg(HACK_DARK)
            };
            spans.push(Span::styled(c.to_string(), style));
        }
        lines.push(Line::from(spans));
    }
    Text::from(lines)
}
