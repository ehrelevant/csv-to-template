use std::env;
use std::fs;
use std::io::Write;

struct FilePaths {
    csv_path: String,
    template_path: String,
    output_folder_path: String,
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_paths = if args.len() == 4 {
        FilePaths {
            csv_path: args[1].clone(),
            template_path: args[2].clone(),
            output_folder_path: args[3].clone(),
        }
    } else {
        println!("Proper command line arguments were not supplied. Please provide the paths to the following files.");

        let mut csv_path = String::new();
        let mut template_path = String::new();
        let mut output_folder_path = String::new();

        print!("CSV file path?: ");
        let _ = std::io::stdout().flush();
        if let Err(_) = std::io::stdin().read_line(&mut csv_path) {
            println!("CSV file path could not be parsed.");
        }

        print!("Template file path?: ");
        let _ = std::io::stdout().flush();
        if let Err(_) = std::io::stdin().read_line(&mut template_path) {
            println!("Template file path could not be parsed.");
            return;
        }

        print!("Destination folder path? (./outputs/): ");
        let _ = std::io::stdout().flush();
        if let Err(_) = std::io::stdin().read_line(&mut output_folder_path) {
            println!("Destination folder path could not be parsed.");
            return;
        }

        FilePaths {
            csv_path: csv_path.trim().to_string(),
            template_path: template_path.trim().to_string(),
            output_folder_path: match output_folder_path.trim() {
                "" => String::from("./outputs/"),
                other => other.to_string(),
            },
        }
    };

    let template_content = match fs::read_to_string(&file_paths.template_path) {
        Ok(content) => content,
        Err(_) => {
            println!("Template file cannot be read. Terminating program");
            return;
        }
    };

    let mut csv_reader = match csv::Reader::from_path(&file_paths.csv_path) {
        Ok(rdr) => rdr,
        Err(_) => {
            println!("CSV file cannot be read. Terminating program...");
            return;
        }
    };

    if let Err(_) = fs::create_dir_all(&file_paths.output_folder_path) {
        println!("Failed to create output directory. Terminating program...");
        return;
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

        let mut output_file =
            match fs::File::create(format!("{}{}.txt", &file_paths.output_folder_path, iii)) {
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

    println!("Operation successful. Terminating program...")
}
