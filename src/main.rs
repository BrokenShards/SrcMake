// main.rs //

use crate::app::print_usage;

mod app;
mod name;
mod paths;

fn main() -> Result<(), ()>
{
	match app::run_srcmake()
	{
		Ok(()) =>
		{
			println!("Srcmake ran successfully.");
			Ok(())
		}
		Err(e) =>
		{
			println!("Srcmake did not run successfully: {e}");
			print_usage();
			Err(())
		}
	}
}
