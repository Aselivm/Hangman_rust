mod current_game;
mod hangman_draw;

use current_game::CurrentGame;
use hangman_draw::display_hangman;
use std::fs::File;
use std::io::{self, BufRead, Write};
use rand::Rng;

fn main() {
    let list_of_words = read_from_file().unwrap_or_else(|err| {
        eprintln!("Не удалось прочитать файл: {}", err);
        std::process::exit(1);
    });
    let random_word = get_random_word(&list_of_words).expect("Словарь пуст");

    let mut current_game = CurrentGame::new(random_word, 6);
    play(&mut current_game);
}

fn play(current_game: &mut CurrentGame) {
    println!("Игра началась!");

    while !current_game.is_game_over() {
        display_hangman(current_game.attempts_left());
        display_game_state(current_game);
        let guess = get_user_guess();

        if current_game.already_guessed(&guess) {
            println!("Эта буква уже была введена.");
            continue;
        }

        check_guess(current_game, guess);
    }

    if current_game.is_won() {
        println!("Поздравляем! Вы угадали слово: {}", current_game.word());
    } else {
        display_hangman(0);
        println!("Вы проиграли. Загаданное слово было: {}", current_game.word());
    }
}

fn display_game_state(current_game: &CurrentGame) {
    println!("Текущее состояние слова: {}", current_game.masked_word());
    println!("Оставшиеся попытки: {}", current_game.attempts_left());
    println!("Введенные буквы: {:?}", current_game.guessed_letters());
}

fn get_user_guess() -> char {
    let mut input = String::new();
    println!();
    print!("Введите букву: ");
    io::stdout().flush().unwrap();

    io::stdin().read_line(&mut input).expect("Не удалось прочитать ввод");
    input.trim().chars().next().unwrap()
}

fn check_guess(current_game: &mut CurrentGame, guess: char) {
    current_game.add_guess(guess);

    if current_game.word_contains(guess) {
        current_game.reveal_letter(guess);
    } else {
        current_game.decrement_attempts();
    }
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

fn get_random_word(words: &Vec<String>) -> Option<String> {
    if words.is_empty() {
        return None;
    }

    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0..words.len());
    Some(words[index].clone())
}
