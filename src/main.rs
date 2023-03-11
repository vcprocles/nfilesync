use std::fs;


fn main() {
    init();
    directory_parse()
}

fn init(){
    config_parse();
}

fn directory_parse() {
    let paths = fs::read_dir("./").unwrap();

    for path in paths {
        println!("Name: {}", path.unwrap().path().display())
    }
}