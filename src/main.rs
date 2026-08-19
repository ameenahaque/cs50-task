use std::io;

fn main() {
    println!("Number:");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    let card = input.trim();

    let mut sum = 0;
    let mut position = 0;

    for digit_char in card.chars().rev() {
        let mut digit = digit_char.to_digit(10).unwrap();

        if position % 2 == 1 {
            digit *= 2;

            if digit >= 10 {
                digit = digit / 10 + digit % 10;
            }
        }

        sum += digit;
        position += 1;
    }

    if sum % 10 != 0 {
        println!("INVALID");
        return;
    }

    let length = card.len();

    if length == 15 && (card.starts_with("34") || card.starts_with("37")) {
        println!("AMEX");
    } else if length == 16
        && ["51", "52", "53", "54", "55"]
            .iter()
            .any(|prefix| card.starts_with(prefix))
    {
        println!("MASTERCARD");
    } else if (length == 13 || length == 16) && card.starts_with("4") {
        println!("VISA");
    } else {
        println!("INVALID");
    }
}