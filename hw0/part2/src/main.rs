// you'll need these two imports to read input form the user
use std::io;
use std::io::Write;

fn main() {
    println!("Welcome to Hangman!");

    let mut s: String = String::new();
    s.push_str("cargo");
    let mut blankword: String = "-".repeat(s.len());
    let mut guesscount = 5;

    while guesscount != 0 {
        if blankword == s {
            println!("You guessed it right the word is {}!", s);
            break;
        }

        println!("The word so far is {}", blankword);
        println!("You have {} guesses left", guesscount);
        // I want to limit this to one character idk how though
        // I just read how to do this and im not doing that you need to use one a crate or two
        // unsafe rust and that is just not for me.

        println!("Please guess a letter: ");
        io::stdout().flush().expect("Error flushing stdout.");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Error reading line");

        // This is really stupid but it works
        //let ths = ["first", "second", "third", "fourth", "fifth"];

        if s.contains(&guess.to_lowercase().trim()) {
            println!("The string does contain {}", guess);
            // I need to get the index of the character that the string contains
            // then replace that char index with the guess
            if let Some(index) = s.find(&guess.to_lowercase().trim()) {
                blankword.replace_range(index..index + 1, &guess.to_lowercase().trim());
            }
        } 
        else 
        {
            println! {"The string doesn't contain {}",guess}
            guesscount -= 1
        }

        // I need to becauses this index needs to go forward not reversed.

        //     let mut charindex = s.len() - guesses;
        //     //println!("{}",charindex);
        //     match s.chars().nth(charindex)
        //     {
        //         Some(c) => {
        //             println!("The {} character is {}",ths[charindex],c);
        //         },
        //         None => {
        //             println!("The String is empty");
        //         }
        //     }
    }
}
