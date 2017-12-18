pub fn str_to_u32_vec(s: &str) -> Vec<u32> {
    s.chars()
        .map(|c|
            c.to_digit(10)
                .unwrap())
        .collect::<Vec<u32>>()
}

pub fn get_rotated_vec(v: Vec<u32>, num_steps: u32) -> Vec<u32> {
    let (a, b) = v.split_at(num_steps as usize);
    let mut spun_chars: Vec<u32> = vec![];
    spun_chars.extend_from_slice(b);
    spun_chars.extend_from_slice(a);

    spun_chars
}

pub fn inverse_captcha_rotate(sequence: Vec<u32>, num_steps: u32) -> u32 {
    // Creates a second vector and rotate it the given number of steps
    // Not very efficient, but it worked the first time around
    // It helped me get the hang of rust iterators

    let chars_clone = sequence.clone();

    let spun_chars = get_rotated_vec(chars_clone, num_steps);

    sequence.into_iter().zip(spun_chars.into_iter())
        .fold(0,
              |acc, pair|
                  if pair.0 == pair.1 { acc + pair.0 } else { acc })
}

pub fn inverse_captcha(sequence: Vec<u32>, num_steps: u32) -> u32 {
    // Use a modulus to determine the correct index to check against
    // instead of creating a whole second vector

    sequence.iter().enumerate()
        .fold(0,
              |acc, e|
                  if e.1 == sequence.get((e.0 + num_steps as usize) % sequence.len())
                      .unwrap() {
                      acc + e.1
                  } else {
                      acc
                  })
}

pub fn inverse_captcha_part_1(sequence: &str) -> u32 {
    inverse_captcha(str_to_u32_vec(sequence), 1)
}

pub fn inverse_captcha_part_2(sequence: &str) -> u32 {
    inverse_captcha(str_to_u32_vec(sequence), sequence.chars().count() as u32/2)
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
