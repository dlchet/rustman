use std::collections::HashSet;

struct Game {
    word: String,
    solved: bool,
    lives: u8,
    correct_guesses: HashSet<char>,
    incorrect_guesses: HashSet<char>,
}

impl Game {
    fn new(word: String) -> Self {
        Game {
            word,
            solved: false,
            lives: 5,
            correct_guesses: vec![].into_iter().collect(),
            incorrect_guesses: vec![].into_iter().collect(),
        }
    }

    fn dec_lives(&mut self) {
        self.lives -= 1
    }
}

pub fn main() {
    let mut game = Game::new("large boulder the size of a small boulder".to_string());
    let word_chars = get_unique_chars(&game.word);

    while game.solved == false && game.lives > 0 {
        draw_game(&game);

        if let Some(i) = get_guess() {
            match test_guess(i, &game) {
                true => {
                    game.correct_guesses.insert(i);
                }
                false => {
                    game.dec_lives();
                    game.incorrect_guesses.insert(i);
                }
            }
        }

        if word_chars == game.correct_guesses {
            game.solved = true;
        }
    }

    draw_game(&game);
}

fn format_secret_word(word: &String, guesses: &HashSet<char>) -> String {
    let mut formatted_string = String::new();

    for c in word.chars() {
        formatted_string.push(if c == ' ' {
            c
        } else if guesses.contains(&c) {
            c
        } else {
            '_'
        })
    }

    formatted_string
}

fn get_guess() -> Option<char> {
    let mut input = String::new();

    println!("Enter a guess.");
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input.");

    input.trim().chars().nth(0)
}

fn test_guess(guess: char, game: &Game) -> bool {
    if game.word.contains(guess) && !game.incorrect_guesses.contains(&guess) {
        true
    } else {
        false
    }
}

fn get_unique_chars(word: &String) -> HashSet<char> {
    let mut result: HashSet<char> = vec![].into_iter().collect();
    let split_word: String = word.split_whitespace().collect();

    for char in split_word.chars() {
        result.insert(char);
    }

    result
}

fn draw_game(game: &Game) {
    clear();

    if game.solved {
        println!("Game solved!");
    } else {
        match game.lives {
            0 => {
                println!(
                    "┌───────────────
│           ▼
│         ┌───┐
│         │x x│
│         └─┬─┘
│           │
│     ┌─────┼─────┐
│     │     │     │
│           │
│           │
│           │
│           │
│        ┌──┴──┐
│        │     │
│        │     │
│        │     │
│        │     │
│        │     │
│       ─┘     └─
│"
                );
            }
            1 => {
                println!(
                    "┌───────────────
│           ▼
│         ┌───┐
│         │   │
│         └─┬─┘
│           │
│     ┌─────┼─────┐
│     │     │     │
│           │
│           │
│           │
│           │
│        ┌──┴──┐
│        │     │
│        │     │
│        │     │
│        │     │
│        │     │
│       ─┘     └─
│"
                );
            }
            2 => {
                println!(
                    "┌───────────────
│           ▼
│         ┌───┐
│         │   │
│         └─┬─┘
│           │
│           │
│           │
│           │
│           │
│           │
│           │
│        ┌──┴──┐
│        │     │
│        │     │
│        │     │
│        │     │
│        │     │
│       ─┘     └─
│"
                )
            }
            3 => {
                println!(
                    "┌───────────────
│           ▼
│         ┌───┐
│         │   │
│         └─┬─┘
│           │
│           │
│           │
│           │
│           │
│           │
│           │
│
│
│
│
│
│
│
│"
                )
            }
            4 => {
                println!(
                    "┌───────────────
│           ▼
│         ┌───┐
│         │   │
│         └───┘
│
│
│
│
│
│
│
│
│
│
│
│
│
│
│"
                );
            }
            _ => {
                println!(
                    "┌───────────────
│           ▼
│
│
│
│
│
│
│
│
│
│
│
│
│
│
│
│
│
│"
                );
            }
        }
        println!(
            "\n{}",
            format_secret_word(&game.word, &game.correct_guesses)
        );
        println!("\n{} lives left.", game.lives)
    }
}

fn clear() {
    // Clear terminal screen and place cursor at first row & column
    print!("\x1B[2J\x1B[1;1H");
}
