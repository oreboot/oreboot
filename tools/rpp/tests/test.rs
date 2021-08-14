use rpp;

use std::fs;
use std::fs::File;
use std::io::Write;

use std::env::temp_dir;
use uuid::Uuid;

fn tmp_filename() -> String {
    let mut dir = temp_dir();
    dir.push(Uuid::new_v4().to_string());

    dir.to_string_lossy().into_owned()
}

#[test]
fn simple_include() {
    let main = tmp_filename();
    let inc = tmp_filename();

    let test_string = format!("#include \"{}\"", inc);
    let include_string = "keep";

    let mut main_file = File::create(&main).unwrap();
    let mut inc_file = File::create(&inc).unwrap();

    main_file.write_all(test_string.as_bytes()).unwrap();
    inc_file.write_all(include_string.as_bytes()).unwrap();

    let mut rpp_ctx = rpp::Context::new();
    let out: String = rpp::process_str(&test_string, &mut rpp_ctx).unwrap();

    fs::remove_file(main).unwrap();
    fs::remove_file(inc).unwrap();

    assert!(out.contains("keep"));
}
