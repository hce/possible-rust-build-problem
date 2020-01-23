use std::io::Write;

fn main() {
    let mut f = std::fs::File::create("demo.txt").unwrap();
    println!("cargo:warning=demo.txt was created!");
    f.write_all("Hello Demo Content".as_bytes()).unwrap();
    if let Err(e) = std::fs::File::open("demo.txt") {
        println!("File::create succeeded, but file::open did not: {:?}", e);
    }
}
