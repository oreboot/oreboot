fn main() {
    println!("cargo:rerun-if-changed=fixed-dtfs.dts");
    println!("cargo:rerun-if-changed=link.ld");
}
