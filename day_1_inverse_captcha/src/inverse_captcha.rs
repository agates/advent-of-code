
pub fn inverse_captcha(sequence: &str) -> u32 {
    let chars = sequence.chars()
        .map(|c| c.to_digit(10).unwrap()).collect::<Vec<u32>>();
    let chars_clone = chars.clone();

    let (a, b) = chars_clone.split_at(1);
    let mut spun_chars: Vec<u32> = vec![];
    spun_chars.extend_from_slice(b);
    spun_chars.extend_from_slice(a);

    chars.into_iter().zip(spun_chars.into_iter())
        .fold(0,
              |acc, pair|
                  if pair.0 == pair.1 { acc + pair.0 } else { acc })
}

#[cfg(test)]
mod tests {
    use inverse_captcha::inverse_captcha;

    #[test]
    fn sequence_1() {
        assert_eq!(inverse_captcha("1122"), 3)
    }

    #[test]
    fn sequence_2() {
        assert_eq!(inverse_captcha("1111"), 4)
    }

    #[test]
    fn sequence_3() {
        assert_eq!(inverse_captcha("1234"), 0)
    }

    #[test]
    fn sequence_4() {
        assert_eq!(inverse_captcha("91212129"), 9)
    }
}
