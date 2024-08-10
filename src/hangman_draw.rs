pub fn display_hangman(attempts_left: i32) {
    let stages = [
        "
  +---+
  |   |
      |
      |
      |
      |
=========",
        "
  +---+
  |   |
  O   |
      |
      |
      |
=========",
        "
  +---+
  |   |
  O   |
  |   |
      |
      |
=========",
        "
  +---+
  |   |
  O   |
 /|   |
      |
      |
=========",
        "
  +---+
  |   |
  O   |
 /|\\  |
      |
      |
=========",
        "
  +---+
  |   |
  O   |
 /|\\  |
 /    |
      |
=========",
        "
  +---+
  |   |
  O   |
 /|\\  |
 / \\  |
      |
=========",
    ];

    let index = 6 - attempts_left;
    println!("{}", stages[index as usize]);
}
