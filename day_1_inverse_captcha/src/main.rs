mod inverse_captcha;

fn main() {
    use inverse_captcha::*;

    println!("The answer is {}", inverse_captcha_part_2(
        "1111"
    ))
}