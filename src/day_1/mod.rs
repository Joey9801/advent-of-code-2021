pub fn parse_input(raw: &str) -> Vec<u16> {
    raw.lines().map(|line| line.parse().unwrap()).collect()
}

pub fn part_1(input: &[u16]) -> u16 {
    let mut count = 0;
    let mut previous = input[0];
    for num in &input[1..] {
        if previous < *num {
            count += 1;
        }
        previous = *num;
    }
    count
}

pub fn part_2(numbers: &[u16]) -> u16 {
    let mut count = 0;
    for i in 0..numbers.len() - 3 {
        if numbers[i] < numbers[i + 3] {
            count += 1;
        }
    }

    count
}

pub fn part_2_unchecked(numbers: &[u16]) -> u16 {
    let mut count = 0;
    unsafe {
        for i in 0..numbers.len() - 3 {
            if numbers.get_unchecked(i) < numbers.get_unchecked(i + 3) {
                count += 1;
            }
        }
    }

    count
}

impl_day!("2021", "1", "Sonar Sweep", Vec<u16>, u16, u16);
