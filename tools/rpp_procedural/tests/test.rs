use rpp_procedural::preprocess_asm;

#[test]
fn simple_preprocess() {
    let out = preprocess_asm!("tests/testdata/simple.S");

    assert!(out.contains("mov $1, %rax"));
}

#[test]
fn inject_macros() {
    let out = preprocess_asm!(
        "tests/testdata/external_macros.S",
        "EXTERNAL1=100",
        "EXTERNAL2=200"
    );

    println!("{}", out);

    assert!(out.contains("mov $100, %rax"));
    assert!(out.contains("mov $200, %rax"));
}
