//! Yakka Remote
//!
//! This is the remote controller application.

use std::{
    fmt::Debug,
    io,
    net::{Ipv4Addr, SocketAddr, SocketAddrV4},
    sync::{Arc, RwLock},
    time::Duration,
};

use clap::Parser;
use ratatui::{
    Frame, Terminal,
    backend::CrosstermBackend,
    crossterm::{
        ExecutableCommand,
        event::{self, Event, KeyCode, KeyEvent},
        terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
    },
    layout::{Constraint, Direction, Layout},
    text::Text,
    widgets::{Block, Paragraph},
};

use log::{Level, LevelFilter, Log, error, info};

use anyhow::Result;
use tokio::net::{TcpSocket, TcpStream};

#[derive(Debug, Clone)]
pub struct BlockLogger(Arc<RwLock<String>>);

impl Log for BlockLogger {
    #[inline]
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &log::Record) {
        let &Self(ref target_buffer) = self;

        let mut target_handle = target_buffer.write().expect("failed to lock log buffer");

        target_handle.push_str(
            format!(
                "{} - {}: {}",
                record.level(),
                record.target(),
                record.args()
            )
            .as_str(),
        );

        target_handle.push('\n');
    }

    #[inline]
    fn flush(&self) {
        /* no-op */
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let log_buffer = Arc::new(RwLock::new(String::new()));

    let logger = Box::leak(Box::new(BlockLogger(log_buffer.clone())));

    let _ = log::set_logger(logger);

    log::set_max_level(LevelFilter::Info);

    terminal::enable_raw_mode()?;

    io::stdout().execute(EnterAlternateScreen)?;

    let target_backend = CrosstermBackend::new(io::stdout());

    let mut target_terminal = Terminal::new(target_backend)?;

    let mut target_context = Context {
        log_buffer,
        ..Default::default()
    };

    loop {
        target_terminal.autoresize()?;

        target_terminal.draw(|frame| target_context.draw(frame))?;

        if event::poll(Duration::from_millis(200))? {
            if let Event::Key(KeyEvent { code: key_code, .. }) = event::read()? {
                match key_code {
                    KeyCode::Char(target_char) => target_context.command_buffer.push(target_char),
                    KeyCode::Esc => break,
                    KeyCode::Tab => {
                        let mut log_handle = target_context
                            .log_buffer
                            .write()
                            .expect("failed to lock log buffer");

                        log_handle.clear()
                    }
                    KeyCode::Enter => target_context.execute_command().await?,
                    KeyCode::Backspace => target_context.backspace(),
                    KeyCode::Up => {
                        target_context.log_buffer_scroll_vertical =
                            target_context.log_buffer_scroll_vertical.saturating_sub(1)
                    }
                    KeyCode::Down => {
                        target_context.log_buffer_scroll_vertical =
                            target_context.log_buffer_scroll_vertical.saturating_add(1)
                    }
                    KeyCode::Left => {
                        target_context.log_buffer_scroll_horizontal = target_context
                            .log_buffer_scroll_horizontal
                            .saturating_sub(1)
                    }
                    KeyCode::Right => {
                        target_context.log_buffer_scroll_horizontal = target_context
                            .log_buffer_scroll_horizontal
                            .saturating_add(1)
                    }
                    _ => (),
                }
            }
        }
    }

    terminal::disable_raw_mode()?;

    io::stdout().execute(LeaveAlternateScreen)?;

    Ok(())
}

/// The application context.
#[derive(Debug, Default)]
pub struct Context {
    command_buffer: String,

    log_buffer: Arc<RwLock<String>>,

    log_buffer_scroll_vertical: u16,
    log_buffer_scroll_horizontal: u16,

    current_connection: Option<TcpStream>,
}

impl Context {
    async fn execute_command(&mut self) -> Result<()> {
        let &mut Self {
            ref mut command_buffer,
            ref mut current_connection,
            ..
        } = self;

        match Command::try_parse_from(command_buffer.split_whitespace()) {
            Ok(target_command) => match target_command {
                Command::Connect { address, port } => {
                    info!("connect @ {address}:{port}");

                    let addr = SocketAddr::V4(SocketAddrV4::new(address, port));

                    let socket = TcpSocket::new_v4()?;

                    *current_connection = Some(socket.connect(addr).await?);
                }
            },
            Err(..) => error!("failed to parse command"),
        }

        command_buffer.clear();

        Ok(())
    }

    fn backspace(&mut self) {
        let &mut Self {
            ref mut command_buffer,
            ..
        } = self;

        let mut target_iter = command_buffer.chars();

        let _ = target_iter.next_back();

        *command_buffer = target_iter.collect();
    }

    fn draw(&self, frame: &mut Frame) {
        let &Self {
            ref command_buffer,
            ref log_buffer,
            ref current_connection,
            log_buffer_scroll_vertical,
            log_buffer_scroll_horizontal,
        } = self;

        let &[log_area, command_area, status_area, ..] = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(1), Constraint::Max(3), Constraint::Max(3)])
            .split(frame.area())
            .as_ref()
        else {
            unreachable!()
        };

        let log_handle = log_buffer.read().expect("failed to lock log buffer");

        let log_widget = Paragraph::new(Text::raw(log_handle.as_str()))
            .block(Block::bordered().title("Logs"))
            .scroll((log_buffer_scroll_vertical, log_buffer_scroll_horizontal));

        frame.render_widget(log_widget, log_area);

        let command_widget = Paragraph::new(Text::raw(command_buffer))
            .left_aligned()
            .block(Block::bordered().title("Command Line"));

        frame.render_widget(command_widget, command_area);

        let status_str = if let Some(..) = current_connection {
            "Connected"
        } else {
            "Not Connected"
        };

        let status_widget = Paragraph::new(Text::raw(status_str))
            .block(Block::bordered().title("Operational Status"));

        frame.render_widget(status_widget, status_area);
    }
}

#[derive(Parser, Debug, Clone)]
#[clap(name = "remote")]
pub enum Command {
    #[command(name = "connect")]
    Connect {
        #[arg(short = 'A')]
        address: Ipv4Addr,

        #[arg(short = 'p', default_value = "5353")]
        port: u16,
    },
}
