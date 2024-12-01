pub fn parse_input(path: &str) -> String {
    let path = format!("{}/input/{path}", env!("CARGO_MANIFEST_DIR"));
    std::fs::read_to_string(path).expect("input file is missing")
}
