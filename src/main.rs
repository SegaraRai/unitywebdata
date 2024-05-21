use clap::{command, Parser};

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(help = "The file to extract")]
    file: String,
}

const UWD_SIGNATURE: &[u8] = b"UnityWebData1.0\0";

fn main() {
    let args = Args::parse();
    let dirname = std::path::Path::new(&args.file)
        .file_stem()
        .unwrap()
        .to_str()
        .unwrap();

    // Read the bytes from the file
    let bytes = std::fs::read(&args.file).unwrap();

    // Check signature
    if bytes.len() < UWD_SIGNATURE.len() || &bytes[..UWD_SIGNATURE.len()] != UWD_SIGNATURE {
        eprintln!("Invalid file format");
        std::process::exit(1);
    }

    let body_offset = u32::from_le_bytes(bytes[16..20].try_into().unwrap()) as usize;
    println!("Body offset: {body_offset:#010X}");

    // Extract the data
    let mut offset = 20;
    while offset < body_offset {
        let file_offset =
            u32::from_le_bytes(bytes[offset..offset + 4].try_into().unwrap()) as usize;
        offset += 4;
        let file_size = u32::from_le_bytes(bytes[offset..offset + 4].try_into().unwrap()) as usize;
        offset += 4;
        println!("- Offset: {file_offset:#010X}, Size: {file_size:#010X}");

        let file_name_length =
            u32::from_le_bytes(bytes[offset..offset + 4].try_into().unwrap()) as usize;
        offset += 4;
        let file_name = std::str::from_utf8(&bytes[offset..offset + file_name_length]).unwrap();
        offset += file_name_length;
        println!("  Filename: {file_name}");

        let file_data = &bytes[file_offset..file_offset + file_size];
        let file_name = std::path::Path::new(dirname).join(file_name);
        std::fs::create_dir_all(file_name.parent().unwrap()).unwrap();
        std::fs::write(file_name, file_data).unwrap();
    }
}
