use std::env;
use std::fs;
use std::io::Write;

fn main() {
    let args: Vec<String> = env::args().collect();

    let csv_path = &args[1];
    let template_path = &args[2];

    println!("CSV file path: {csv_path}");
    println!("Template file path: {template_path}");

    let template_content = match fs::read_to_string(template_path) {
        Ok(content) => content,
        Err(_) => {
            println!("Template file cannot be read. Terminating program");
            return;
        }
    };

    let mut csv_reader = match csv::Reader::from_path(&csv_path) {
        Ok(rdr) => rdr,
        Err(_) => {
            println!("CSV file cannot be read. Terminating program...");
            return;
        }
    };

    let csv_headers = match csv_reader.headers() {
        Ok(headers) => headers,
        Err(_) => {
            println!("CSV headers cannot be read. Terminating program...");
            return;
        }
    };

    let csv_headers: Vec<_> = csv_headers.iter().map(|x| format!("${{{}}}", x)).collect();

    for (iii, result) in csv_reader.records().enumerate() {
        let record = match result {
            Ok(record) => record,
            Err(_) => {
                println!("Error reading CSV records. Terminating program...");
                return;
            }
        };

        let mut replaced_content = template_content.clone();

        for jjj in 0..csv_headers.len() {
            replaced_content = replaced_content.replace(&csv_headers[jjj], &record[jjj]);
        }

        if let Err(_) = fs::create_dir_all("./src/outputs/") {
            println!("Failed to create output directory. Terminating program...");
            return;
        };

        let mut output_file = match fs::File::create(format!("./src/outputs/{}.txt", iii)) {
            Ok(file) => file,
            Err(_) => {
                println!("Cannot create output file. Terminating program...");
                return;
            }
        };

        if let Err(_) = output_file.write(replaced_content.as_bytes()) {
            println!("Error occured while writing to files. Terminating program...");
            return;
        };
    }
}
