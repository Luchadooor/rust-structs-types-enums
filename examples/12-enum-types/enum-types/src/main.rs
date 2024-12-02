#[derive(Debug)]
enum WineRegions {
    Bordeaux,
    Burgundy,
    Champagne,
    Tuscany,
    Rioja,
    NapaValley,
    Basel
}

struct Wine {
    name: String,
    region: WineRegions, // wine regions used as a type
}

fn supported_regions(w: &WineRegions) {
    match w {
        WineRegions::Rioja => println!("Rioja is supported!"),
        _ => println!("{:?} is not supported!", w),
    }
}

fn main() {
    let wine1 = Wine {
        name: String::from("Chateau Margaux"),
        region: WineRegions::Bordeaux,
    };

    let wine2 = Wine {
        name: String::from("Barolo"),
        region: WineRegions::Tuscany,
    };

    let wine3 = Wine {
        name: String::from("Heisser Hirsch"),
        region: WineRegions::Basel,
    };

    println!("Wine 1: {} from {:?}", wine1.name, wine1.region);
    println!("Wine 2: {} from {:?}", wine2.name, wine2.region);
    println!("Wine 3: {} from {:?}", wine3.name, wine3.region);
    supported_regions(&wine1.region);
    supported_regions(&WineRegions::Rioja);

    let mut msg = Message::Move { x: 10, y: 20 };
    process_message(msg);
    msg = Message::Quit;
    process_message(msg);
    msg = Message::Write("Simsalabim!".to_string());
    process_message(msg);
    msg = Message::ChangeColor(255,255,255);
    process_message(msg);
    msg = Message::AdditionalVariant;
    process_message(msg);

    println!("Popularity: {}", get_popularity(wine1.region));
    println!("Popularity: {}", get_popularity(wine2.region));
    println!("Popularity: {}", get_popularity(wine3.region));

}

#[derive(Debug)]
enum Message {
    Quit,                       // No associated data
    Move { x: i32, y: i32 },    // Struct-like data
    Write(String),              // Tuple-like data
    ChangeColor(u8, u8, u8),    // Tuple-like data
    AdditionalVariant,
}

fn process_message(msg: Message) {
    match msg {
        Message::Quit => println!("Quit message received."),
        Message::Move { x, y } => println!("Move to position: x={}, y={}", x, y),
        Message::Write(text) => println!("Writing message: {}", text),
        Message::ChangeColor(r, g, b) => println!("Changing color to RGB({}, {}, {})", r, g, b),
        _ => println!("Message Variant '{:?}' is not supported.", msg)
    }
}

fn get_popularity(wr: WineRegions) -> String {
    match wr{
        WineRegions::Basel => return String::from("Most popular wine region on the planet!"),
        WineRegions::Bordeaux => return String::from("Most popular wine region in France."),
        _ => return String::from(format!("The wine region of {:?} is nothing compared to the wine region of Basel.", wr)),
    }
}