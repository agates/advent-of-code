
pub fn calc_checksum_part_1(rows: Vec<String>) -> i32 {
    rows.iter()
        .map(|row| row.split_whitespace())
        .map(|row| row.map(|s| s.parse().unwrap()).collect())
        .map(|row:Vec<i32>|
            (row.iter().fold(i32::max_value(),
                      |a, b| if a < *b { a } else { *b }),
             row.iter().fold(i32::min_value(),
                      |a, b| if a > *b { a } else { *b })))
        .map(|row| row.1 - row.0)
        .sum()
}

pub fn calc_checksum_part_2(rows: Vec<String>) -> i32 {

    rows.iter()
        .map(|row| row.split_whitespace())
        .map(|row| row.map(|s| s.parse().unwrap()).collect())
        .flat_map(|row:Vec<i32>|
            row.iter()
                .flat_map(|n|
                    row.iter()
                        .filter(|p| *p != n)
                        .map(|m| *n as f64 / *m as f64)
                        .filter(|&p| p % 1.0 == 0.0)
                        .collect::<Vec<f64>>())
                .collect::<Vec<f64>>())
        .map(|m| m as i32)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_sequence_1() {
        assert_eq!(calc_checksum_part_1(
            vec!["5\t1\t9\t5".to_string(),
                 "7\t5\t3".to_string(),
                 "2\t4\t6\t8".to_string()]),
                   18)
    }

    #[test]
    fn part_2_sequence_1() {
        assert_eq!(calc_checksum_part_2(
            vec!["5\t9\t2\t8".to_string(),
                 "9\t4\t7\t3".to_string(),
                 "3\t8\t6\t5".to_string()]),
                   9)
    }
}
