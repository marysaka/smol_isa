mod ast;

const FILE_DATA: &str = "

    ADD r0 r1
    ADDI r3 123
";

fn main() {
    let tree = ast::parse_source(FILE_DATA);
    println!("{tree:#?}");
}
