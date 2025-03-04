pub fn to_floor(input: &[u8]) -> isize {
    let mut floor = 0;
    input.iter().for_each(|byte| match *byte {
        b'(' => {
            floor += 1;
        }
        b')' => {
            floor -= 1;
        }
        _ => unreachable!(),
    });
    return floor;
}

pub fn first_basement_index(input: &[u8]) -> usize {
    let mut floor = 0;
    for (index, byte) in input.iter().enumerate() {
        match *byte {
            b'(' => {
                floor += 1;
            }
            b')' => {
                floor -= 1;
            }
            _ => unreachable!(),
        }
        if floor < 0 {
            return index + 1;
        }
    }
    return 0;
}

#[cfg(test)]
mod tests {
    use super::*;

    static SAMPLE_1: &[u8] = b"(()(()(";
    static SAMPLE_2: &[u8] = b"()())";
    static INPUT: &[u8] = include_bytes!("./inputs/day1.txt");

    #[test]
    fn part1() {
        assert_eq!(3, to_floor(SAMPLE_1), "sample");
        assert_eq!(74, to_floor(INPUT), "input");
    }

    #[test]
    fn part2() {
        assert_eq!(5, first_basement_index(SAMPLE_2), "sample");
        assert_eq!(1795, first_basement_index(INPUT), "input");
    }
}
