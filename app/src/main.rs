use app::run;
use crossterm::{
	execute,
	terminal::{disable_raw_mode, LeaveAlternateScreen},
};
use std::{
	env,
	io::{stdout, Error},
};

fn main() {
	// Get the name of the file to be opened from the runtime args
	let args: Vec<String> = env::args().collect();
	let filename = String::from(&args[1]);

	// Run the app
	match run(filename) {
		Ok(_) => (),
		Err(run_err) => {
			// Turn off raw mode for stdout (enable canonical mode)
			let raw_mode_err = match disable_raw_mode() {
				Ok(_) => Error::other("Raw mode disabled successfully"),
				Err(err) => err,
			};
			// Exit the alternate screen
			match execute!(stdout(), LeaveAlternateScreen,) {
				Ok(_) => (),
				Err(stdout_err) => panic!("{:?} | {:?}", raw_mode_err, stdout_err),
			}
			panic!("{:?}", run_err);
		}
	};
}
