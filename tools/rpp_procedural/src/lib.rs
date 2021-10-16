use proc_macro::TokenStream;
use rpp::Context;
use std::env;
use std::fmt;
use std::path::Path;

#[derive(Debug, Clone)]
struct BadInputError {
    message: String,
}

impl fmt::Display for BadInputError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl BadInputError {
    pub fn new(message: String) -> Self {
        BadInputError { message }
    }
}

/// Parse an input TokenStream which contains a filename followed by any number
/// of key-value pair macro definitions. Macro definitions will be added to
/// the rpp_ctx.
///
/// Returns the filename.
fn parse_input(input: TokenStream, rpp_ctx: &mut Context) -> Result<String, BadInputError> {
    let input_str = input.to_string();
    let mut args = input_str.split(',');

    let in_file = args
        .next()
        .ok_or_else(|| BadInputError::new(String::from("No filename present")))?;
    let unquoted_file = &in_file.trim()[1..in_file.len() - 1];

    for define in args {
        let trimmed = define.trim();
        let unquoted_define = &trimmed[1..trimmed.len() - 1];
        let mut parts = unquoted_define.splitn(2, '=');
        let key = parts
            .next()
            .ok_or_else(|| BadInputError::new(String::from("Macro definition has no value")))?;

        let value = match parts.next() {
            Some(val) => val.trim(),
            None => "",
        };

        rpp_ctx.add_macro(key, value);
    }

    Ok(unquoted_file.to_string())
}

/// Preprocess the an assembly file with 0 or more macro definitions.
///
/// Returns a string literal with preprocessed asm.
///
/// Sample usage:
/// let asm_string = preprocess_asm!("file.asm", "A=1", "B=2");
#[proc_macro]
pub fn preprocess_asm(input: TokenStream) -> TokenStream {
    let mut rpp_ctx = Context::new();
    let in_file = parse_input(input, &mut rpp_ctx).unwrap();

    let curr_path = env::current_dir().unwrap();
    let path = Path::new(&in_file);

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

    let mut out: String = rpp::process_file(filename, &mut rpp_ctx).unwrap();

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
