use rand::prelude::ThreadRng;
use rand::Rng;
use rand::{self, thread_rng};
use std::io;
use std::process;
//To Do
//Bet on multiple numbers
//Bet on evens(odds)/thirds/1-18/19-36/black&red
//Create every number as a struct/enum?? and add properties to each number
//Payout

struct WheelCompare {
    number: bool,
    green: bool,
    red: bool,
    black: bool,
    even: bool,
    odd: bool,
    first_third: bool,
    second_third: bool,
    third_third: bool,
}

struct UserCompare {
    number: bool,
    green: bool,
    red: bool,
    black: bool,
    even: bool,
    odd: bool,
    first_third: bool,
    second_third: bool,
    third_third: bool,
}
//match guessed_number against Number variables = tick
//e.g. match against all red numbers, if okay turn red: true = tick
//Alternative - Create vector with set values and match against vec = not needed
//guessed_number does not mean red/black/odd/even
//bring in wheel spin and compare inside every function = tick
impl WheelCompare {
    fn number(spin: i32, guessed_number: i32) -> bool {
        if spin == guessed_number {
            println!(
                "Your Number: {}   |  The Wheel's Number: {}",
                guessed_number, spin
            );
            return true;
        } else {
            println!("Did not hit your number!");
            return false;
        }
    }

    fn green(spin: i32) -> bool {
        match spin {
            0 => {
                println!("Wheel hit: Green");
                return true;
            }
            _ => {
                println!("Wheel did not hit: Green");
                return false;
            }
        }
    }

    fn red(spin: i32) -> bool {
        match spin {
            32 | 19 | 21 | 25 | 34 | 27 | 36 | 30 | 23 | 5 | 16 | 1 | 14 | 9 | 18 | 7 | 12 | 3 => {
                println!("Wheel hit: Red");
                return true;
            }
            _ => {
                println!("Wheel did not hit: Red");
                return false;
            }
        }
    }

    fn black(spin: i32) -> bool {
        match spin {
            15 | 4 | 2 | 17 | 6 | 13 | 11 | 8 | 10 | 24 | 33 | 20 | 31 | 22 | 29 | 28 | 35 | 26 => {
                println!("Wheel hit: Black");
                return true;
            }
            _ => {
                println!("Wheel did not hit: Black");
                return false;
            }
        }
    }

    fn even(spin: i32) -> bool {
        match spin {
            2 | 4 | 6 | 8 | 10 | 12 | 14 | 16 | 18 | 20 | 22 | 24 | 26 | 28 | 30 | 32 | 34 | 36 => {
                println!("Wheel hit: Even");
                return true;
            }
            _ => {
                println!("Wheel did not hit: Even");
                return false;
            }
        }
    }

    fn odd(spin: i32) -> bool {
        match spin {
            1 | 3 | 5 | 7 | 9 | 11 | 13 | 15 | 17 | 19 | 21 | 23 | 25 | 27 | 29 | 31 | 33 | 35 => {
                println!("Wheel hit: Odd");
                return true;
            }
            _ => {
                println!("Wheel did not hit: Odd");
                return false;
            }
        }
    }

    fn first_third(spin: i32) -> bool {
        match spin {
            1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 => {
                println!("\nWheel hit: First Third");
                return true;
            }
            _ => {
                println!("Wheel did not hit: First Third");
                return false;
            }
        }
    }

    fn second_third(spin: i32) -> bool {
        match spin {
            13 | 14 | 15 | 16 | 17 | 18 | 19 | 20 | 21 | 22 | 23 | 24 => {
                println!("Wheel hit: Second third");
                return true;
            }
            _ => {
                println!("Wheel did not hit: Second third");
                return false;
            }
        }
    }

    fn third_third(spin: i32) -> bool {
        match spin {
            25 | 26 | 27 | 28 | 29 | 30 | 31 | 32 | 33 | 34 | 35 | 36 => {
                println!("Wheel hit: Third third");
                return true;
            }
            _ => {
                println!("Wheel did not hit: Third third");
                return false;
            }
        }
    }
}

impl UserCompare {
    fn number(spin: i32, guessed_number: i32) -> bool {
        if guessed_number == spin {
            println!("User hit Number!!!!!");
            return true;
        } else {
            return false;
        }
    }

    fn green(guessed_number: i32) -> bool {
        match guessed_number {
            0 => {
                //println!("User hit: Green");
                return true;
            }
            _ => {
                //println!("User did not hit: Green");
                return false;
            }
        }
    }

    fn red(guessed_number: i32) -> bool {
        match guessed_number {
            32 | 19 | 21 | 25 | 34 | 27 | 36 | 30 | 23 | 5 | 16 | 1 | 14 | 9 | 18 | 7 | 12 | 3 => {
                println!("\nUser hit: Red");
                return true;
            }
            _ => {
                //println!("User did not hit: Red");
                return false;
            }
        }
    }

    fn black(guessed_number: i32) -> bool {
        match guessed_number {
            15 | 4 | 2 | 17 | 6 | 13 | 11 | 8 | 10 | 24 | 33 | 20 | 31 | 22 | 29 | 28 | 35 | 26 => {
                println!("User hit: Black");
                return true;
            }
            _ => {
                //println!("User did not hit: Black");
                return false;
            }
        }
    }

    fn even(guessed_number: i32) -> bool {
        match guessed_number {
            2 | 4 | 6 | 8 | 10 | 12 | 14 | 16 | 18 | 20 | 22 | 24 | 26 | 28 | 30 | 32 | 34 | 36 => {
                println!("\nUser hit: Even");
                return true;
            }
            _ => {
                //println!("User did not hit: Even");
                return false;
            }
        }
    }

    fn odd(guessed_number: i32) -> bool {
        match guessed_number {
            1 | 3 | 5 | 7 | 9 | 11 | 13 | 15 | 17 | 19 | 21 | 23 | 25 | 27 | 29 | 31 | 33 | 35 => {
                //println!("\nUser hit: Odd");
                return true;
            }
            _ => {
                //println!("User did not hit: Odd");
                return false;
            }
        }
    }

    fn first_third(guessed_number: i32) -> bool {
        match guessed_number {
            1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 => {
                println!("\nUser hit: First Third");
                return true;
            }
            _ => {
                //println!("\nUser did not hit: First Third");
                return false;
            }
        }
    }

    fn second_third(guessed_number: i32) -> bool {
        match guessed_number {
            13 | 14 | 15 | 16 | 17 | 18 | 19 | 20 | 21 | 22 | 23 | 24 => {
                println!("\nUser hit: Second third");
                return true;
            }
            _ => {
                //println!("User did not hit: Second third");
                return false;
            }
        }
    }

    fn third_third(guessed_number: i32) -> bool {
        match guessed_number {
            25 | 26 | 27 | 28 | 29 | 30 | 31 | 32 | 33 | 34 | 35 | 36 => {
                println!("\nUser hit: Third third");
                return true;
            }
            _ => {
                //println!("User did not hit: Third third");
                return false;
            }
        }
    }
}
//SORT THIRDS PAYOUT
fn main() {
    //Starting Balance
    let balance = 1000;

    //Welcoming Function
    welcome();
    //Different parts of roulette board guessing section
    let guessed_number: i32 = number_guess().trim().parse().unwrap();
    let red_or_black: String = red_or_black();
    let even_or_odd: String = even_or_odd();
    let thirds: String = thirds();

    //Each parts betting amount (if any)
    println!("Balance: {}", balance);
    let number_amount_bet: i32 = number_bet_amount().trim().parse().unwrap();
    let rnb_amount_bet: i32 = rnb_bet_amount().trim().parse().unwrap();
    let eoo_amount_bet: i32 = eoo_bet_amount().trim().parse().unwrap();
    let thirds_amount_bet: i32 = thirds_bet_amount().trim().parse().unwrap();
    //Generates RGN
    let spin = wheel_spin();

    //Runs comparisons between user input and wheel spin
    //POTENTIALLY OBSOLETE IF USER DOES NOT BET ON SINGULAR NUMBER
    //UserCompare obsolete.
    let user_input = user_compare(guessed_number, spin);
    let wheel_outcome = wheel_outcome(spin, guessed_number);

    //Failsafe to prevent user betting more than they have and outside roulette range
    if number_amount_bet > balance
        || rnb_amount_bet > balance
        || eoo_amount_bet > balance
        || thirds_amount_bet > balance
        || guessed_number > 36
        || guessed_number < 0
    {
        println!("Incorrect Input. ABORTING");
        process::abort();
    } else {
        //Compares user and wheel results and provides boolean output.
        let number_check = check_number(&user_input, &wheel_outcome);
        let green_check = check_green(&user_input, &wheel_outcome);
        let rnb_check = check_red_or_black(&user_input, &wheel_outcome);
        let even_or_odd_check = check_even_or_odd(&user_input, &wheel_outcome);
        let thirds_check = check_thirds(&user_input, &wheel_outcome);
        //Issue is no error handling if user types anything other than e.g. red/black/n
        //Calculates winnings if consists with parameters.
        let number_winnings: i32 = number_winnings(number_check, guessed_number, number_amount_bet);
        let green_winnings: i32 = green_winnings(green_check, guessed_number, number_amount_bet);
        let rnb_winnings: i32 = rnb_winnings(rnb_check, red_or_black, rnb_amount_bet);

        let eoo_winnings: i32 = eoo_winnings(even_or_odd_check, even_or_odd, eoo_amount_bet);

        let thirds_winnings: i32 = thirds_winnings(thirds_check, thirds, thirds_amount_bet);

        println!(
            "Your new balance: {}",
            balance = balance
                + number_winnings
                + green_winnings
                + rnb_winnings
                + eoo_winnings
                + thirds_winnings
        );
    }
}
//Welcoming text, informs user how to not bet on certain catagories
fn welcome() {
    println!("Welcome to Roulette!");
    println!("If you do not wish to bet on a certain catagory, please type n");
}
//Randomly generates a number
fn wheel_spin() -> i32 {
    let mut rng: ThreadRng = thread_rng();
    let chance: i32 = rng.gen_range(0..36);
    println!("The wheel number is: {}", chance);
    return chance;
}

//Assigns boolean result values to user_input.X
fn user_compare(spin: i32, guessed_number: i32) -> UserCompare {
    let user_input = UserCompare {
        number: UserCompare::number(spin, guessed_number),
        green: UserCompare::green(guessed_number),
        red: UserCompare::red(guessed_number),
        black: UserCompare::black(guessed_number),
        even: UserCompare::even(guessed_number),
        odd: UserCompare::odd(guessed_number),
        first_third: UserCompare::first_third(guessed_number),
        second_third: UserCompare::second_third(guessed_number),
        third_third: UserCompare::third_third(guessed_number),
    };
    return user_input;
}
//Assigns boolean result values to wheel_outcome.X
fn wheel_outcome(spin: i32, guessed_number: i32) -> WheelCompare {
    let wheel_outcome = WheelCompare {
        number: WheelCompare::number(spin, guessed_number),
        green: WheelCompare::green(spin),
        red: WheelCompare::red(spin),
        black: WheelCompare::black(spin),
        even: WheelCompare::even(spin),
        odd: WheelCompare::odd(spin),
        first_third: WheelCompare::first_third(spin),
        second_third: WheelCompare::second_third(spin),
        third_third: WheelCompare::third_third(spin),
    };
    return wheel_outcome;
}
//Remove wheel_outcome
//Calculates winnings, grabs the user input boolean check, wheel boolean result, amount bet and the guess itself.
fn number_winnings(number_check: bool, guessed_number: i32, number_amount_bet: i32) -> i32 {
    let new_balance: i32;
    if (number_check == true) && guessed_number.to_string() != "n".to_string() {
        //Win
        new_balance = number_amount_bet * 36;
        println!("Number Winnings: {}", new_balance);
        return new_balance;
    } else {
        //Lose
        return -(number_amount_bet);
    }
}

fn green_winnings(green_check: bool, guessed_number: i32, number_amount_bet: i32) -> i32 {
    let new_balance: i32;
    if (green_check == true) && (guessed_number == 0) {
        new_balance = number_amount_bet * 36;
        println!("Green Winnings: {}", new_balance);
        return new_balance;
    } else {
        return -(number_amount_bet);
    }
}

fn rnb_winnings(rnb_check: bool, red_or_black: String, rnb_amount_bet: i32) -> i32 {
    let new_balance: i32;
    if (rnb_check == true) && (red_or_black != "n") {
        new_balance = rnb_amount_bet * 2;
        println!("RNB Winnings: {}", new_balance);
        return new_balance;
    } else {
        //Lose
        return -(rnb_amount_bet);
    }
}

fn eoo_winnings(even_or_odd_check: bool, even_or_odd: String, eoo_amount_bet: i32) -> i32 {
    let new_balance: i32;
    if (even_or_odd_check == true) && (even_or_odd != "n") {
        new_balance = eoo_amount_bet * 2;
        println!("EOO Winnings: {}", new_balance);
        return new_balance;
    } else {
        //Lose
        return -(eoo_amount_bet);
    }
}

fn thirds_winnings(thirds_check: bool, thirds: String, thirds_amount_bet: i32) -> i32 {
    let new_balance: i32;
    if (thirds_check == true) && (thirds != "n") {
        new_balance = thirds_amount_bet * 3;
        println!("Thirds Winnings: {}", new_balance);
        return new_balance;
    } else {
        return -(thirds_amount_bet);
    }
}

//if user and wheel are the same, return true
fn check_number(user_input: &UserCompare, wheel_outcome: &WheelCompare) -> bool {
    if (user_input.number == true) && (wheel_outcome.number == true) {
        return true;
    } else {
        return false;
    }
}

fn check_green(user_input: &UserCompare, wheel_outcome: &WheelCompare) -> bool {
    if (user_input.green == true) && (wheel_outcome.green == true) {
        //println!("Green!");
        return true;
    } else {
        //println!("Not hit!");
        return false;
    }
}

fn check_red_or_black(user_input: &UserCompare, wheel_outcome: &WheelCompare) -> bool {
    if (user_input.red == true) && (wheel_outcome.red == true) {
        //println!("\nRed Match!");
        return true;
    } else if (user_input.black == true) && (wheel_outcome.black == true) {
        //println!("Black Match!");
        return true;
    } else {
        //println!("\nNot hit!");
        return false;
    }
}

fn check_even_or_odd(user_input: &UserCompare, wheel_outcome: &WheelCompare) -> bool {
    if (user_input.even == true) && (wheel_outcome.even == true) {
        //println!("Even Match!");
        return true;
    } else if (user_input.odd == true) && (wheel_outcome.odd == true) {
        //println!("Odd Match!");
        return true;
    } else {
        //println!("Not hit!");
        return false;
    }
}

fn check_thirds(user_input: &UserCompare, wheel_outcome: &WheelCompare) -> bool {
    if (user_input.first_third == true) && (wheel_outcome.first_third == true) {
        //println!("First Third Match!");
        return true;
    } else if (user_input.second_third == true) && (wheel_outcome.second_third == true) {
        //println!("Second Third Match!");
        return true;
    } else if (user_input.third_third == true) && (wheel_outcome.third_third == true) {
        //println!("Third Third Match!");
        return true;
    } else if (user_input.green == true) && (wheel_outcome.green == true) {
        //println!("Green!");
        return false;
    } else {
        //println!("Not hit!");
        return false;
    }
}
///////

//Basic I/O, Asks for guess and bet amount
fn number_guess() -> String {
    println!("Type your number guess here: ");
    let mut body: String = String::new();
    match io::stdin().read_line(&mut body) {
        Ok(_) => {}
        Err(error) => println!("error: {}", error),
    }
    return body;
}

fn number_bet_amount() -> String {
    println!("How much would you like to bet on your number: ");
    let mut body: String = String::new();
    match io::stdin().read_line(&mut body) {
        Ok(_) => {}
        Err(error) => println!("error: {}", error),
    }
    return body;
}

fn red_or_black() -> String {
    println!("Red or Black: ");
    let mut body: String = String::new();
    match io::stdin().read_line(&mut body) {
        Ok(_) => {}
        Err(error) => println!("error: {}", error),
    }
    return body;
}

fn rnb_bet_amount() -> String {
    println!("How much would you like to bet on red or black: ");
    let mut body: String = String::new();
    match io::stdin().read_line(&mut body) {
        Ok(_) => {}
        Err(error) => println!("error: {}", error),
    }
    return body;
}

fn even_or_odd() -> String {
    println!("Even or Odd: ");
    let mut body: String = String::new();
    match io::stdin().read_line(&mut body) {
        Ok(_) => {}
        Err(error) => println!("error: {}", error),
    }
    return body;
}

fn eoo_bet_amount() -> String {
    println!("How much would you like to bet on even or odd: ");
    let mut body: String = String::new();
    match io::stdin().read_line(&mut body) {
        Ok(_) => {}
        Err(error) => println!("error: {}", error),
    }
    return body;
}

fn thirds_bet_amount() -> String {
    println!("How much would you like to bet on thirds: ");
    let mut body: String = String::new();
    match io::stdin().read_line(&mut body) {
        Ok(_) => {}
        Err(error) => println!("error: {}", error),
    }
    return body;
}

fn thirds() -> String {
    println!("First, second or third third: ");
    let mut body: String = String::new();
    match io::stdin().read_line(&mut body) {
        Ok(_) => {}
        Err(error) => println!("error: {}", error),
    }
    return body;
}
