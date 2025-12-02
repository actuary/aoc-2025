enum Rotation {
    L(i64),
    R(i64),
}

fn parse(data: &str) -> Vec<Rotation> {
    data.trim()
        .split("\n")
        .map(|line| {
            if line.starts_with('L') {
                Rotation::L(line[1..].parse().unwrap())
            } else {
                Rotation::R(line[1..].parse().unwrap())
            }
        })
        .collect()
}

pub fn part1(data: &str) -> i64 {
    let rotations = parse(data);
    let dial_start = 50;
    let result = rotations.iter().scan(dial_start, |acc, val| {
        match val {
            Rotation::L(i) => *acc = (*acc - i).rem_euclid(100),
            Rotation::R(i) => *acc = (*acc + i).rem_euclid(100),
        };

        Some(*acc)
    });

    result.filter(|v| *v == 0).count() as i64
}

pub fn solve(data: &str, dial_start: i64) -> i64 {
    let rotations = parse(data);
    rotations
        .iter()
        .scan((dial_start, 0), |acc, val| {
            match val {
                Rotation::L(0) => {}
                Rotation::R(0) => {}
                Rotation::L(i) => {
                    let rem_i = i.rem_euclid(100);
                    let acc_0 = (acc.0 - i).rem_euclid(100);
                    let acc_1 = acc.1 + i.div_euclid(100);
                    let acc_1 = if acc.0 != 0 && acc.0 - i <= 0 && (rem_i >= acc.0) {
                        acc_1 + 1
                    } else {
                        acc_1
                    };

                    *acc = (acc_0, acc_1);
                }
                Rotation::R(i) => {
                    let rem_i = i.rem_euclid(100);
                    let acc_0 = (acc.0 + i).rem_euclid(100);
                    let acc_1 = acc.1 + i.div_euclid(100);
                    let acc_1 = if acc.0 != 0 && acc.0 + i >= 100 && (rem_i + acc.0 >= 100) {
                        acc_1 + 1
                    } else {
                        acc_1
                    };

                    *acc = (acc_0, acc_1);
                }
            };

            Some(*acc)
        }).last().unwrap().1 as i64
}

pub fn part2(data: &str) -> i64 {
    solve(data, 50)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_works() {
        let data = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";
        assert_eq!(part1(data), 3);
    }


    #[test]
    fn part2_works() {
        let data = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";
        assert_eq!(part2(data), 6);
    }
}
