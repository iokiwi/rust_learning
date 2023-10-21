use rand::Rng;

// fn greet_world() {
//     println!("Hello, world!");
//     let southern_germany = "Grüß Gott!";
//     let japan = "ハロー・ワールド";
//     let regions = [southern_germany, japan];
//     for region in regions.iter() {
//         println!("{}", &region);
//     }
// }

// fn fizz_buzz(n: i32) {
//     for i in 1..n {
//         if i % 15 == 0 {
//             println!("fizzbuzz");
//         } else if i % 3 == 0 {
//             println!("fizz");
//         } else if i % 5 == 0 {
//             println!("buzz");
//         } else {
//             println!("{}", i);
//         }
//     }
// }

fn number_guessing(limit: u8) -> u8 {

    use std::io::{stdin, stdout, Write};

    let mut rng = rand::thread_rng();
    let answer: u8 = rng.gen_range(1..limit+1);
    let mut guess_count: u8 = 0;

    println!("Guess a number between 1 and {}", limit);

    loop {

        let mut s: String=String::new();

        print!("Guess {}: ", guess_count+1);
        let _ = stdout().flush();
        stdin().read_line(&mut s).expect("Error");

        let trimmed = s.trim();
        let mut guess: u8 = 0;

        match trimmed.parse::<u8>() {
            Ok(i) => {
                guess = i;
                guess_count += 1;
            },
            Err(e) => println!("Could not convert string to integer: {}", e),
        }

        if guess > answer {
            println!("Too High!");
        } else if guess < answer {
            println!("Too Low!");
        } else {
            println!("Correct!");
            return 0;
        }
    }
}

fn main() {
    // fizz_buzz(20);
    // greet_world();
    number_guessing(10);
}
