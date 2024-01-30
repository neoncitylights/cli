use crosstui_prompt::prompt_yesno;
use std::io::Result;

fn main() -> Result<()> {
	let _result1 = prompt_yesno("Would you like to use TypeScript?")?;
	let _result2 = prompt_yesno("Would you like to use Rust?")?;
	Ok(())
}
