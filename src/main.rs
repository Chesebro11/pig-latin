//I think I was heavily over complicating this originally. This is much more simple
//Thank what I was planning.
// Made sure to account for Capital/lowercase consonants

use std::io;

fn main () {
    println!("Type a word to be translated: ");

    let mut input = String::new();
    io::stdin()
    .read_line(&mut input)
    .unwrap();
    match input.chars().nth(0).unwrap() {
        'a' | 'A' | 'e' | 'E' |'i' |'I' |'o' |'O' |'U' |'u' | 'y' | 'Y' =>
        println!("{}-hay", input.trim()),
        c => println!("{}-{}ay ", &input[c.len_utf8()..].trim(), c)
    }
}