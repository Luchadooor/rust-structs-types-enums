use std::env;
// use std::io::{Error, Read};

#[derive(Debug)]
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

fn print_converted(parts: &Vec<&str>) {
    let number_raw = parts.get(0);
    let scale = parts.get(1);
    if let None = scale {return;}
    if let Some(n) = number_raw {
        let cast = n.parse::<u64>();
        if let Ok(size) = cast {
            println!("Nummer gefunden");
            // scale ist hier nicht None
            match scale.unwrap() {
                &"bytes" => println!("Converted: {}", format_size(size)),
                &"kilobytes" => println!("Converted: {}", format_size(size * 1000)),
                &"megabytes" => println!("Converted: {}", format_size(size * 1000000)),
                &"gigabytes" => println!("Converted: {}", format_size(size * 1000000000)),
                &"terabytes" => println!("Converted: {}", format_size(size * 1000000000000)),
                _ => println!("Unbekannte Einheit ({}) für Anzahl: {}", &scale.unwrap(), size),
            }

        }
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

    let args: Vec<String> = env::args().collect();

    // The first argument is the size that was used to call the program. Must use quotes to
    // read this as a single argument
    println!("Size is: {:?}.", args.get(1));

    if let Some(n) = &args.get(1) {
        let parts: Vec<&str> = n.split(' ').collect();
        if parts.len() > 1 {
            print_converted(&parts);
            println!("Len: {}", parts.len());

        }
        println!("Juhu: {}", parts.len());
    }

}
