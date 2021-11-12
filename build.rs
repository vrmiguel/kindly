use cc;

fn main() {

    println!("cargo:rustc-link-lib=crypt");

    cc::Build::new().file("cc/helper.c").compile("helper");

    println!("cargo:rerun-if-changed=cc/helper.c");
}
