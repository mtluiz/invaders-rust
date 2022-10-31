use std::error::Error;
use rusty_audio::Audio;
use std::io;
use std::{
    sync::mpsc::{self, Receiver},
    time::{Duration, Instant},
    {thread},
};
use crossterm::{
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
    event::{Event, self, KeyCode},
    cursor::{Hide, Show}
};

fn main() -> Result <(), Box<dyn Error>> {
    let mut audio = Audio::new();
    audio.add("explode", "sounds/explode.wav");   
    audio.add("lose", "sounds/lose.wav");
    audio.add("move", "sounds/move.wav");
    audio.add("pew", "sounds/pew.wav");
    audio.add("startup", "sounds/startup.wav");
    audio.add("win", "sounds/win.wav");
    audio.play("startup");

    let mut stdout = io::stdout();
    terminal::enable_raw_mode()?;
    stdout.execute(EnterAlternateScreen)?;
    stdout.execute(Hide)?;

    'gameloop: loop{
        while event::poll(Duration::default())? {
            if let Event::key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Esc | KeyCode::Char('q') => {
                        audio.play("lose");
                        break 'gameloop;
                    }

                    _ => {}
                }
            }
        }
    }

    audio.wait();
    stdout.execute(Show)?;
    stdout.execute(LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;
    Ok(())
}
