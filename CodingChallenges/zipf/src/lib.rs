mod error;
mod file;
mod logic;

use error::Error;
use file::File;
use logic::encoder;
use logic::decoder;

pub fn process(args: &Vec<String>) {
    if args.len() < 2 {
        error::exit(
            "Usage: ./zipf path/to/file",
            Error::NoArgs 
        );
    }

    let input_file = file::read_file(&args[1]);

    let compressed_file_path = compress(input_file);
    let _ = decompress(file::read_file(&compressed_file_path));

    println!("Done!");
}

fn compress(input_file: File) -> String {
    println!("Compressing {}...", &input_file.file_path);

    let output_file_path = format!("{}.zipf", &input_file.file_path);
    let content_bytes = encoder::encode_contents(input_file);
    file::write_file(&output_file_path, &content_bytes);

    println!("File compressed and saved to {}", output_file_path);
    return output_file_path;
}

fn decompress(input_file: File) -> String {
    println!("Decompressing {}...", input_file.file_path);

    let decoded_string: String = decoder::decode_contents(&input_file);
    let output_file_path = input_file.file_path.replace(".zipf", ".unzipf");
    file::write_file(&output_file_path, decoded_string.as_bytes());

    println!("File decompressed and saved to {}", output_file_path);
    return output_file_path;
}
