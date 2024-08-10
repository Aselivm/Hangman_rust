mod current_game;
mod hangman_draw;
mod read_file;

use read_file::get_random_word;
use current_game::CurrentGame;
use hangman_draw::display_hangman;
use std::io::{self, Write};

fn main() {
    let random_word = get_random_word().expect("Словарь пуст");

    let mut current_game = CurrentGame::new(random_word);
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

