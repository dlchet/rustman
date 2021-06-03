struct Game {
    word: String,
    solved: bool,
    lives: u8,
    correct_guesses: usize,
    chars_guessed: Vec<char>,
}

impl Game {
    fn new(word: String) -> Game {
        Game {
            word,
            solved: false,
            lives: 5,
            correct_guesses: 0,
            chars_guessed: Vec::new(),
        }
    }
}

pub fn main() {
    let mut game = Game::new("rustman".to_string());

    while game.solved == false && game.lives > 0 {
        draw_game(&game);

        if let Some(i) = get_guess() {
            game.chars_guessed.push(i);

            match test_guess(i, &game.word) {
                true => {
                    game.correct_guesses += 1;
                }
                false => {
                    game.lives -= 1;
                }
            }
        }

        if game.word.chars().count() == game.correct_guesses {
            game.solved = true;
            draw_game(&game);
        }
    }
}

fn format_secret_word(word: &String, mask: &Vec<char>) -> String {
    let mut formatted_string = String::new();

    for c in word.chars() {
        formatted_string.push(if c == ' ' {
            c
        } else if mask.contains(&c) {
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

fn test_guess(guess: char, word: &String) -> bool {
    if word.contains(guess) {
        true
    } else {
        false
    }
}

fn draw_game(game: &Game) {
    clear();
    println!("{}", format_secret_word(&game.word, &game.chars_guessed));

    if game.solved {
        println!("Game solved!");
    } else {
        match game.lives {
            0 => {
                println!("Game over!");
            }
            1 => {
                println!("1 life left!");
            }
            2 => {
                println!("2 lives left.");
            }
            3 => {
                println!("3 lives left.");
            }
            4 => {
                println!("4 lives left.");
            }
            _ => {
                println!("{} lives left.", game.lives)
            }
        }
    }
}

fn clear() {
    // Clear terminal screen and place cursor at first row & column
    print!("\x1B[2J\x1B[1;1H");
}
