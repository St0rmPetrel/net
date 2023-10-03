fn main() {
    println!("cargo:rerun-if-changed=src/rawsock.c");
    cc::Build::new().file("src/rawsock.c").compile("rawsock");
}
