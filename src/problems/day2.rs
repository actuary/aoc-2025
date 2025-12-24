use std::collections::HashSet;

fn parse(data: &str) -> Vec<(i64, i64)> {
    data.trim()
        .split(",")
        .map(|line| {
            let ids = line.split("-");
            let ids_int: Vec<i64> = ids.map(|val| val.parse().unwrap()).collect();
            (ids_int[0], ids_int[1])
        })
        .collect()
}

pub fn is_invalid_id(val: i64) -> bool {
    if val < 10 {
        return false;
    }

    let val_str = val.to_string();

    let midpoint = val_str.len() / 2;

    let left = &val_str[0..midpoint];
    let right = &val_str[midpoint..];
    left == right
}

pub fn part1(data: &str) -> i64 {
    let result: i64 = parse(data)
        .iter()
        .map(|&(first, last)| (first..=last).filter(|&v| is_invalid_id(v)))
        .flatten()
        .sum();

    result
}

pub fn is_invalid_id_part2(val: i64) -> bool {
    if val < 10 {
        return false;
    }

    let val_str = val.to_string();
    for i in (1..=(((val_str.len() as f64).sqrt() as usize) + 2)).rev() {
        if i == val_str.len() {
            continue;
        }

        if val_str.len() % i == 0 {
            if i == 1 {
                let unique_chars: HashSet<char> = val_str.chars().collect();
                return unique_chars.len() == 1;
            }

            let like = &val_str[..i];
            let mut is_invalid = true;
            for j in (i..val_str.len()).step_by(i) {
                let curr = &val_str[j..j + i];
                if like != curr {
                    is_invalid = false;
                    break;
                }
            }

            if is_invalid {
                return is_invalid;
            }
        }
    }

    false
}

pub fn part2(data: &str) -> i64 {
    let result: i64 = parse(data)
        .iter()
        .map(|&(first, last)| (first..=last).filter(|&v| is_invalid_id_part2(v)))
        .flatten()
        .sum();

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_works() {
        let data = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        assert_eq!(part1(data), 1227775554);
    }

    #[test]
    fn part2_works() {
        let data = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        assert_eq!(part2(data), 4174379265);
    }
}
