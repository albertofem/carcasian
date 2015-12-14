extern crate argparse;

use self::argparse::{ArgumentParser, Store};

/// We use this to parse program arguments
pub fn parse_host_args(host: &mut String, port: &mut String, description: &str) -> ()
{
	let mut ap = ArgumentParser::new();

	ap.set_description(description);

	ap.refer(host).add_option(
		&["-h", "--host"],
		Store,
		"127.0.0.1"
	);

	ap.refer(port).add_option(
		&["-p", "--port"],
		Store,
		"8991"
	);

	ap.parse_args_or_exit();
}