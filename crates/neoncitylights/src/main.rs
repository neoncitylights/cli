use clap::builder::PossibleValuesParser;
use clap::{crate_authors, crate_version, Arg, Command as ClapCommand};
use std::process::Command;

fn main() {
	let cli = cli();
	let matches = cli.get_matches();
	match matches.subcommand() {
		Some(("config", submatches)) => match submatches.subcommand() {
			Some(("set", _submatches)) => {}
			Some(("get", _submatches)) => {}
			Some(("list", _submatches)) => {}
			_ => unreachable!(),
		},
		Some(("new", _submatches)) => {}
		Some(("version", _submatches)) => {
			let commit_hash = Command::new("git")
				.args(&["rev-parse", "--short", "HEAD"])
				.output()
				.expect("failed to execute git command")
				.stdout;

			let commit_hash_str = std::str::from_utf8(&commit_hash)
				.expect("failed to convert commit hash to string")
				.trim_end();

			println!("neoncitylights {} ({})", crate_version!(), commit_hash_str);
		}
		_ => unreachable!(),
	}
}

#[allow(non_upper_case_globals)]
fn cli() -> ClapCommand {
	ClapCommand::new("neoncitylights")
		.version("1.0.0")
		.author(crate_authors!(",\n"))
		.about("Samantha's personal command-line tool.")
		.subcommand_required(true)
		.arg_required_else_help(true)
		.help_template(root_help_template())
		.subcommands(subcommands())
}

fn subcommands() -> [ClapCommand; 3] {
	let subcmd_config = ClapCommand::new("config")
		.about("Configure default settings for project creation")
		.help_template(help_template_subcommand())
		.arg_required_else_help(true)
		.subcommands([
			ClapCommand::new("set")
				.about("Set a default setting")
				.help_template(help_template_subcommand())
				.subcommand_precedence_over_arg(true),
			ClapCommand::new("get")
				.about("Get a default setting")
				.help_template(help_template_subcommand()),
			ClapCommand::new("list")
				.about("List all default settings")
				.help_template(help_template_subcommand()),
		]);

	let submcd_create = ClapCommand::new("new")
		.aliases(["create"])
		.about("Create a new project from a template")
		// .help_template(help_template_subcommand())
		.arg(Arg::new("name").help("Name of the project"))
		.arg(Arg::new("language")
			.help("Main programming language to use for the project")
			.value_parser(PossibleValuesParser::new([
				"ts",
				"typescript",
				"rust",
				"php",
				"c",
				"c++",
			])))
		.arg(Arg::new("kind")
			.help("kind of project to create")
			.value_parser(PossibleValuesParser::new(["app", "lib", "executable"])));

	let subcmd_version = ClapCommand::new("version").about("Current version of the CLI");

	[subcmd_config, submcd_create, subcmd_version]
}

fn help_template_subcommand() -> &'static str {
	color_print::untagged!(
		"\
		{about}\n\
		\nUsage:  {usage}\n\
		\nCommands:\n{subcommands}\n\
		\nOptions:\n{options}\n\
		"
	)
}

fn root_help_template() -> &'static str {
	color_print::cstr!(
		"\
		Author: \t{author}\n\
		Source code: \t<blue>https://github.com/neoncitylights/cli</blue>\n\n\
		{about}\n\
		\nUsage:\n  {usage}\n\
		\nCommands:\n{subcommands}\n\
		"
	)
}
