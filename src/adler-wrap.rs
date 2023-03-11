use simd_adler32::Adler32;
use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;


fn calculate_adler(file_path :&String)->u32{
    
}

fn calculate_adler_bufreader(file_path :&String)-> u32 {
    let f = BufReader::new(File::open(file_path).expect("Error opening file"));
    let mut adler = Adler32::new();
    for byte in f.bytes(){
        let readbyte = byte.expect("Error reading file");
        adler.write(&readbyte.to_le_bytes());
    }
    let hash = adler.finish();
    return hash;
}
fn calculate_adler_fsread(file_path :&String)-> u32{
    let filecontent = fs::read(file_path)
        .expect("error reading file");
    let mut adler = Adler32::new();
    adler.write(&filecontent);
    let hash=adler.finish();
    return hash
}