use proc_macro::TokenStream;
use std::env;
use std::path::Path;

#[proc_macro]
pub fn preprocess_asm(input: TokenStream) -> TokenStream {
    let mut rpp_ctx = rpp::Context::new();
    let in_file = input.to_string();
    let p = &in_file[1..in_file.len() - 1];

    let curr_path = env::current_dir().unwrap();
    let path = Path::new(&p);

    // Create a path to the asm directory
    let mut asm_dir = env::current_dir().unwrap();
    asm_dir.push(path.to_path_buf());

    // Change directory to the directory the asm file is in so all #include
    // directives are relative to the root asm file.
    match env::set_current_dir(asm_dir.parent().unwrap()) {
        Ok(()) => (),
        Err(e) => panic!(
            "Couldn't change directory to {}, error = {}",
            asm_dir.parent().unwrap().to_str().unwrap(),
            e
        ),
    };

    let path_buf = path.to_path_buf();
    let filename = &path_buf.file_name().unwrap().to_str().unwrap();

    let mut out: String = rpp::process_file(&filename, &mut rpp_ctx).unwrap();

    // All files in rpp_ctx.processed_files are relative to `in_file`.
    // To ensure cargo can find the files, output their absolute filepaths.
    for f in rpp_ctx.processed_files {
        let abs_path = asm_dir.parent().unwrap().join(f);
        println!("cargo:rerun-if-changed={}", abs_path.to_str().unwrap());
    }

    // This is to be used as a string literal. Escape all quote characters
    out = out.replace("\"", "\\\"");

    // Quote the string literal
    out.insert(0, '"');
    out.insert(out.len(), '"');

    // Change working directory back
    env::set_current_dir(curr_path).unwrap();

    out.parse().unwrap()
}
