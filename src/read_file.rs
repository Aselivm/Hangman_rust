use std::fs::File;
use std::io;
use std::io::BufRead;
use rand::Rng;

pub fn get_random_word() -> Option<String> {
    let list_of_words = read_from_file().unwrap_or_else(|err| {
        eprintln!("Не удалось прочитать файл: {}", err);
        std::process::exit(1);
    });

    if list_of_words.is_empty() {
        return None;
    }

    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0..list_of_words.len());
    Some(list_of_words[index].clone())
}

fn read_from_file() -> io::Result<Vec<String>> {
    let path = "words.txt";

    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    let mut lines = Vec::new();

    for line in reader.lines() {
        let line = line?;
        lines.push(line);
    }
    Ok(lines)
}
