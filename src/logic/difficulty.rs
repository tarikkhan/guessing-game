use std::io;

pub fn choose_difficulty() -> std::io::Result<i32> {
    println!("Guessing Game!");
    println!("Choose difficulty: type 1 for Easy, 2 for Medium, and 3 for Hard");

    let mut counter: u32 = 0;
    loop {
        let mut diff_choice = String::new();
        io::stdin().read_line(&mut diff_choice)?;
        let error_msg = format!("Please type in a number (1 = Easy, 2 = Medium, 3 = Hard)");
        counter += 1;

        let diff_choice = match diff_choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("{error_msg}");
                continue;
            }
        };
        if counter > 5 {
            println!("You get Easy Difficulty: You Have 30 guesses");
            return Ok(30);
        }
        match diff_choice {
            1 => {
                println!("You Chose Easy Difficulty: You Have 30 guesses");
                return Ok(30);
            }
            2 => {
                println!("You Chose Medium Difficulty: You Have 15 guesses");
                return Ok(15);
            }
            3 => {
                println!("You Chose Hard Difficulty: You Have 7 guesses");
                return Ok(7);
            }
            _ => {
                println!("{error_msg}");
                continue;
            }
        };
    }
}
