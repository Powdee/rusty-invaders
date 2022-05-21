use rusty_audio::Audio;
use std::error::Error;

use crossterm::terminal;
use std::io;

use crossterm::cursor::{Hide, Show};
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::ExecutableCommand;

fn main() -> Result<(), Box<dyn Error>> {
	let mut audio = Audio::new();

	audio.add("explode", "explode.wav");
	audio.add("lose", "lose.wav");
	audio.add("move", "move.wav");
	audio.add("pew", "pew.wav");
	audio.add("startup", "startup.wav");
	audio.add("win", "win.wav");

	audio.play("startup");

	// terminal
	let mut stdout = io::stdout();
	terminal::enable_raw_mode()?;
	stdout.execute(EnterAlternateScreen)?;
	stdout.execute(Hide)?;

	// cleanup
	audio.wait();
	stdout.execute(Show)?;
	stdout.execute(LeaveAlternateScreen)?;

	terminal::disable_raw_mode()?;

	return Ok(());
}
