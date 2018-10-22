extern crate bible_reference_rs;

fn main() {
    let refs = bible_reference_rs::parse("Gen 1:1-3, 6 & Acts 1-2");
    println!("{:?}", refs);
}
