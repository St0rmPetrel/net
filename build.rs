fn main() {
    println!("cargo:rerun-if-changed=src/rawsock.c");
    cc::Build::new()
        .file("src/net/rawsock.c")
        .compile("rawsock");
}
