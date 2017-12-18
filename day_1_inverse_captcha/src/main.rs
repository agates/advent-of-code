mod inverse_captcha;

fn main() {
    use inverse_captcha::inverse_captcha;

    println!("The answer is {}", inverse_captcha(
        "1111"
    ))
}