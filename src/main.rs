extern crate vial;
extern crate cmd_lib;

use cmd_lib::run_cmd;

vial::routes! {
	GET "/*path" => |_| { let _ = run_cmd!("./script.sh"); format!("lol") };
	GET "/" => |_| { let _ = run_cmd!("./script.sh"); format!("lol") };
}

fn main() {
	vial::run!().unwrap();
}
