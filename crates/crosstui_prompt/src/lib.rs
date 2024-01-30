use crossterm::{
	event::{read, Event, KeyCode, KeyEvent},
	style::{style, StyledContent, Stylize},
	terminal::{disable_raw_mode, enable_raw_mode},
};
use std::io::{stdout, Result, Write};

pub fn prompt_yesno(question: &str) -> Result<bool> {
	let choices = &["yes", "no"];
	prompt(question, choices, " {} ", |s| s.green().bold(), |s| s.grey()).map(|s| s == choices[0])
}

#[rustfmt::skip]
pub fn prompt<
	OnSelected,
	OnUnselected,
>(
	question: &str,
	options: &[&str],
	delimiter: &str,
	on_selected: OnSelected,
	on_unselected: OnUnselected,

) -> Result<String>
where 
	OnSelected: Fn(StyledContent<&str>) -> StyledContent<&str>,
	OnUnselected: Fn(StyledContent<&str>) -> StyledContent<&str>
{
	enable_raw_mode()?;

	let mut selected = 0;
	loop {
		print!("\r{} ", question);
		for (i, option) in options.iter().enumerate() {
			if i == selected {
				print!("{}", on_selected(style(option)));
			} else {
				print!("{}", on_unselected(style(option)));
			}
			if i != options.len() - 1 {
				print!("{}", delimiter);
			}
		}
		stdout().flush()?;

		match read()? {
			Event::Key(KeyEvent { code: KeyCode::Right, ..}) |
			Event::Key(KeyEvent { code: KeyCode::Down, ..})
			=> {
				selected = (selected + 1) % options.len();
			}
			Event::Key(KeyEvent { code: KeyCode::Left, ..}) |
			Event::Key(KeyEvent { code: KeyCode::Up, .. })
			=> {
				selected = if selected > 0 { selected - 1 } else { options.len() - 1 };
			}
			Event::Key(KeyEvent { code: KeyCode::Enter, ..}) => break,
			_ => {}
		}
	}

	disable_raw_mode()?;
	println!();
	Ok(options[selected].to_string())
}
