use rpp::Context;
use std::env;
use std::path::Path;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Error: No file specified");
        println!("Usage: rpp <path/to/file>");
        process::exit(1);
    }

    let filename = &args[1];

    let mut ctx: Context = Context::new();

    let mut path = Path::new(filename).to_path_buf();
    let path_copy = path.to_owned();
    let name = path_copy.file_name().unwrap().to_str().unwrap();
    path.pop();
    env::set_current_dir(path).unwrap();

    println!("{}", rpp::process_file(name, &mut ctx).unwrap());
}
