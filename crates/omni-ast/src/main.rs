use asdl::Asdl;
pub fn main() {
    let file_path = concat!(env!("CARGO_MANIFEST_DIR"), "/src/omni.asdl");
    let asdl_code = std::fs::read_to_string(file_path).unwrap();
    let ast = Asdl::parse(&asdl_code).unwrap();

    println!("{:#?}", ast);
}
