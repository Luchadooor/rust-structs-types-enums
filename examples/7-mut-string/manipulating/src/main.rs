fn main() {
    let sentence = "the quick brown fox jumps over the lazy dog".to_string();
    count_vowels(&sentence);
    let longest_word = get_longest_word(&sentence);
    println!("The longest word is: {}", longest_word);

    let shortest_word = get_shortest_word(&sentence);
    println!("The shortest word is: {}", shortest_word);


    let word_list = get_all_words_of_length(&sentence, get_length_of_longest_word(&sentence));
    println!("Words with length of longest word: {:#?}", word_list);

    let shortest_words = get_all_words_of_length(&sentence, get_length_of_shortest_word(&sentence));
    println!("Words with length of shortest word: {:#?}", shortest_words);

    let mut sentence2 = "the quick brown fox Adam jumps over the lazy dog Emily".to_string();
    contrabass2(&mut sentence2);
    // Use slicing to get the first three characters of the sentence
    println!("{}", &sentence[0..=4]);

    // concatenate using format!
    let description = format!("Title: Quick story\n{}", sentence);
    //println!("{}", description);

    // iterate over the characters in the sentence
    // for c in sentence.chars() {
    //     match c {
    //         'a' | 'e' | 'i' | 'o' | 'u' => println!("Got a vowel!"),
    //         _ => continue,
    //     }
    // }

    // Split and collect into a vector
    //let words: Vec<&str> = sentence.split_whitespace().collect();
    let words = sentence.split(' ').collect::<Vec<&str>>();
    //println!("{:?}", words);

    let reversed = sentence.chars().rev().collect::<String>();
    println!("{}", reversed);
}

fn count_vowels(sentence: &String) {
    let mut count_a: u32 = 0;
    let mut count_e: u32 = 0;
    let mut count_i: u32 = 0;
    let mut count_o: u32 = 0;
    let mut count_u: u32 = 0;
    for mut c in sentence.chars() {
        match c {
            'a' | 'A' => count_a += 1,
            'e' | 'E' => count_e += 1,
            'i' | 'I' => count_i += 1,
            'o' | 'O' => count_o += 1,
            'u' | 'U' => count_u += 1,
            _ => continue,
        }
    }
    println!("Numer of a or A: {}", count_a);
    println!("Numer of e or E: {}", count_e);
    println!("Numer of i or I: {}", count_i);
    println!("Numer of o or O: {}", count_o);
    println!("Numer of u or U: {}", count_u);
}

fn contrabass2(sentence: &mut String) {
    // Create a new string with replaced vowels
    let modified = sentence.chars().map(|c| match c {
        'a' | 'e' | 'i' | 'o' | 'u' => 'o',
        'A' | 'E' | 'I' | 'O' | 'U' => 'O',
        _ => c,
    }).collect::<String>();

    // Replace the original string's contents
    *sentence = modified;

    println!("Sieben Chinesen: {}", sentence);
}

fn get_longest_word(sentence: &String) -> &str {
    let words = sentence.split_whitespace().collect::<Vec<&str>>();
    let longest = words.iter().max_by_key(|w| w.len()).unwrap();
    longest
}
fn get_shortest_word(sentence: &String) -> &str {
    let words = sentence.split_whitespace().collect::<Vec<&str>>();
    let longest = words.iter().min_by_key(|w| w.len()).unwrap();
    longest
}

fn get_length_of_longest_word(sentence: &String) -> usize {
    let longest = get_longest_word(sentence);
    longest.len()
}

fn get_length_of_shortest_word(sentence: &String) -> usize {
    let shortest = get_shortest_word(sentence);
    shortest.len()
}

fn get_all_words_of_length(sentence: &String, length: usize) -> Vec<&str> {
    let words = sentence.split_whitespace().collect::<Vec<&str>>();
    words.iter().filter_map(|w| if w.len() == length { Some(*w) } else { None }).collect()
}