fn input() -> Vec<u16> {
    include_str!("../input.txt")
        .lines()
        .map(|line| u16::from_str_radix(line, 2))
        .collect::<Result<Vec<_>, _>>()
        .unwrap()
}

fn part_1(input: &[u16]) -> u32 {
    let gamma = {
        let mut one_freqs = [0u16; 12];
        for line in input.iter() {
            for bit in 0..12 {
                if line & (1 << bit) != 0 {
                    one_freqs[bit] += 1;
                }
            }
        }

        let half = input.len() as u16 / 2;

        let mut gamma = 0u16;
        for bit in 0..12 {
            if one_freqs[bit] >= half {
                gamma |= 1 << bit;
            }
        }
        gamma
    };
    let epsilon = !gamma;
    gamma as u32 * epsilon as u32
}

fn part_2(input: &[u16], bits: u16) -> u32 {
    fn find(input: &[u16], bits: u16, set_bit: impl Fn(u16, u16) -> bool) -> u16 {
        let mut val = 0u16;

        for bit in 0..bits {
            let bit = bits - 1 - bit;
            let existing_mask = 0xFFFFu16 << (bit + 1);

            // NB, could replace these three iterations with one regular loop.
            // The compiler may well notice that it could do that on its own anyway.
            let count = input
                .iter()
                .filter(|line| **line & existing_mask == val)
                .count() as u16;
            let ones = input
                .iter()
                .filter(|line| **line & existing_mask == val)
                .filter(|line| **line & (1 << bit) > 0)
                .count() as u16;

            if count == 1 {
                return *input
                    .iter()
                    .filter(|line| *line & existing_mask == val)
                    .next()
                    .unwrap();
            }

            if set_bit(ones, count - ones) {
                val |= 1 << bit;
            }
        }

        val
    }

    let oxy = find(input, bits, |ones, zeros| ones >= zeros) as u32;
    let co2 = find(input, bits, |ones, zeros| ones < zeros) as u32;
    oxy * co2
}

fn main() {
    let input = input();

    dbg!(part_1(&input));
    dbg!(part_2(&input, 12));
}

#[cfg(test)]
mod test {
    use super::*;

    fn test_input() -> &'static [u16] {
        &[
            0b00100u16, 0b11110u16, 0b10110u16, 0b10111u16, 0b10101u16, 0b01111u16, 0b00111u16,
            0b11100u16, 0b10000u16, 0b11001u16, 0b00010u16, 0b01010u16,
        ]
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(test_input(), 5), 230)
    }
}
