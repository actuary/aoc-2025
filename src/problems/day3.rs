use std::collections::HashMap;

struct Bank {
    #[allow(dead_code)]
    batteries: HashMap<u32, Vec<u32>>,
}

fn largest_joltage_helper(
    bank: &Bank,
    position: i64,
    batteries_left: u32,
    line_length: i64,
) -> Option<(i64, u32)> {
    for digit in (1..=9).rev() {
        let result = match bank.batteries.get_key_value(&digit) {
            Some((_, vec_pos)) => {
                let possibilities: Vec<&u32> = vec_pos
                    .iter()
                    .filter(|&&v| v as i64 > position && v < line_length as u32 - batteries_left)
                    .collect();
                if possibilities.len() > 0 {
                    Some(**possibilities.first().unwrap())
                } else {
                    None
                }
            }
            None => None,
        };

        match result {
            Some(pos) => {
                return Some((pos as i64, digit));
            }
            None => (),
        }
    }
    None
}

// probably solve like sudoku? but we are maximising (this is ok because
// )
// terminating condition: digits are length 2
// need to repeat if gone down a dead end, with next battery.
// position, bank, found digits
// if len(digits) == 2 {
//      return digits
// }
//
// for 0..batteries {
//      if pos_of_battery > pos {
//          let sub_digits(pos_of_battery)
//          if sub-digits is None return None
//          return [battery_digit] + sub_digits
//      }
// }
// return None
//
//
fn largest_joltage(bank: &Bank, batteries: u32, line_length: i64) -> Vec<u32> {
    // an earlier position is always better. only need to look
    // at the first position you find. the positions are in order.
    let mut position: i64 = -1;

    let mut digits: Vec<u32> = Vec::new();

    for i in 0..batteries {
        match largest_joltage_helper(bank, position, batteries - i - 1, line_length) {
            Some((new_pos, digit)) => {
                digits.push(digit);
                position = new_pos;
            }
            None => (),
        };
    }

    digits
}

fn parse(data: &str) -> (Vec<Bank>, i64) {
    let mut line_length: i64 = 0;
    let banks: Vec<Bank> = data
        .trim()
        .split("\n")
        .map(|line| {
            let mut batteries: HashMap<u32, Vec<u32>> = HashMap::new();
            line_length = line.len() as i64;
            for (pos, _battery) in line
                .chars()
                .map(|c| c.to_digit(10).unwrap() as u32)
                .enumerate()
            {
                if batteries.contains_key(&_battery) {
                    batteries.get_mut(&_battery).unwrap().push(pos as u32);
                } else {
                    batteries.insert(_battery, vec![pos as u32]);
                };
            }

            Bank { batteries }
        })
        .collect();
    (banks, line_length)
}

pub fn part1(data: &str) -> i64 {
    let (banks, line_length) = parse(data);

    let result: i64 = banks
        .iter()
        .map(|bank| {
            let digits = largest_joltage(bank, 2, line_length);
            let mut sum: u32 = 0;
            for digit in &digits {
                sum = sum * 10 + digit;
            }
            sum as i64
        })
        .sum();

    result
}

pub fn part2(data: &str) -> i64 {
    let (banks, line_length) = parse(data);

    let result: i64 = banks
        .iter()
        .map(|bank| {
            let digits = largest_joltage(bank, 12, line_length);
            let mut sum: i64 = 0;
            for digit in &digits {
                sum = sum * 10 + (*digit as i64);
            }
            sum
        })
        .sum();

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_works() {
        let data = "987654321111111
811111111111119
234234234234278
818181911112111";
        assert_eq!(part1(data), 357);
    }

    #[test]
    fn part2_works() {
        let data = "987654321111111
811111111111119
234234234234278
818181911112111";
        assert_eq!(part2(data), 3121910778619);
    }
}
