use std::fs;

// struct Log{
//     level: String, // &str
//     method: String,
//     path: String
// }

fn main() {
    let log_file_path: &str = "log.txt";
    let content: String = fs::read_to_string(log_file_path).expect("REASON");

    for line in content.lines(){
        if line.contains("ERROR"){ println!("{}", line); }
    }

    println!("--end main--");
}
