enum FileSize {
    Bytes(u64),
    Kilobytes(f64),
    Megabytes(f64),
    Gigabytes(f64),
    Terabytes(f64),
}

fn format_size(size: u64) -> String {
    let filesize = match size {
        0..=999 => FileSize::Bytes(size),
        1000..=999_999 => FileSize::Kilobytes(size as f64 / 1000.0),
        1_000_000..=999_999_999 => FileSize::Megabytes(size as f64 / 1_000_000.0),
        1_000_000_000..=999_999_999_999 => FileSize::Gigabytes(size as f64 / 1_000_000_000.0),
        _ => FileSize::Terabytes(size as f64 / 1_000_000_000_000.0),
    };

    match filesize {
        FileSize::Bytes(bytes) => format!("{} bytes", bytes),
        FileSize::Kilobytes(kb) => format!("{:.2} KB", kb),
        FileSize::Megabytes(mb) => format!("{:.2} MB", mb),
        FileSize::Gigabytes(gb) => format!("{:.2} GB", gb),
        FileSize::Terabytes(gb) => format!("{:.2} TB", gb),
    }
}

fn real_format_size(size: u64) -> String {
    let filesize = match size {
        0..=999 => FileSize::Bytes(size),
        1000..=999_999 => FileSize::Kilobytes(size as f64 / 1024.0),
        1_000_000..=999_999_999 => FileSize::Megabytes(size as f64 / 1024.0 / 1024.0),
        1_000_000_000..=999_999_999_999 => FileSize::Gigabytes(size as f64 / 1024.0 / 1024.0 / 1024.0),
        _ => FileSize::Terabytes(size as f64 / 1024.0 / 1024.0 / 1024.0 / 1024.0),
    };

    match filesize {
        FileSize::Bytes(bytes) => format!("{} bytes", bytes),
        FileSize::Kilobytes(kb) => format!("{:.2} KB", kb),
        FileSize::Megabytes(mb) => format!("{:.2} MB", mb),
        FileSize::Gigabytes(gb) => format!("{:.2} GB", gb),
        FileSize::Terabytes(gb) => format!("{:.2} TB", gb),
    }
}


fn main() {
    // let result = format_size(6888837399);
    // println!("{}", result);
    println!("{}", format_size(12));
    println!("{}", format_size(12000));
    println!("{}", format_size(12000000));
    println!("{}", format_size(12000000000));
    println!("{}", format_size(12000000000000));
    println!("{}", format_size(12000000000000000));

    println!("{} -> {}", format_size(12), real_format_size(12));
    println!("{} -> {}", format_size(12000), real_format_size(12000));
    println!("{} -> {}", format_size(12000000), real_format_size(12000000));
    println!("{} -> {}", format_size(12000000000), real_format_size(12000000000));
    println!("{} -> {}", format_size(12000000000000), real_format_size(12000000000000));
    println!("{} -> {}", format_size(12000000000000000), real_format_size(12000000000000000));
    println!("{} -> {}", format_size(2500), real_format_size(2500));



}
