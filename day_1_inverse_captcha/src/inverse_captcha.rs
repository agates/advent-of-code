
pub fn inverse_captcha(sequence: &str, num_steps: u32) -> u32 {
    let chars = sequence.chars()
        .map(|c| c.to_digit(10).unwrap()).collect::<Vec<u32>>();
    let chars_clone = chars.clone();

    let (a, b) = chars_clone.split_at(num_steps as usize);
    let mut spun_chars: Vec<u32> = vec![];
    spun_chars.extend_from_slice(b);
    spun_chars.extend_from_slice(a);

    chars.into_iter().zip(spun_chars.into_iter())
        .fold(0,
              |acc, pair|
                  if pair.0 == pair.1 { acc + pair.0 } else { acc })
}

pub fn inverse_captcha_part_1(sequence: &str) -> u32 {
    inverse_captcha(sequence, 1)
}

pub fn inverse_captcha_part_2(sequence: &str) -> u32 {
    inverse_captcha(sequence, sequence.chars().count() as u32/2)
}

#[cfg(test)]
mod tests {
    use inverse_captcha::*;

    #[test]
    fn part_1_sequence_1() {
        assert_eq!(inverse_captcha_part_1("1122"), 3)
    }

    #[test]
    fn part_1_sequence_2() {
        assert_eq!(inverse_captcha_part_1("1111"), 4)
    }

    #[test]
    fn part_1_sequence_3() {
        assert_eq!(inverse_captcha_part_1("1234"), 0)
    }

    #[test]
    fn part_1_sequence_4() {
        assert_eq!(inverse_captcha_part_1("91212129"), 9)
    }

    #[test]
    fn part_2_sequence_1() {
        assert_eq!(inverse_captcha_part_2("1212"), 6)
    }

    #[test]
    fn part_2_sequence_2() {
        assert_eq!(inverse_captcha_part_2("1221"), 0)
    }

    #[test]
    fn part_2_sequence_3() {
        assert_eq!(inverse_captcha_part_2("123425"), 4)
    }

    #[test]
    fn part_2_sequence_4() {
        assert_eq!(inverse_captcha_part_2("123123"), 12)
    }

    #[test]
    fn part_2_sequence_5() {
        assert_eq!(inverse_captcha_part_2("12131415"), 4)
    }
}
