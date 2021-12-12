#[derive(Clone, Copy)]
struct FishState {
    // Fixed size ring buffer
    counts: [u64; 9],

    // The index of the first element of the ring
    head: usize,
}

impl FishState {
    fn step(&mut self) {
        // Pop the first element of the ring buffer
        // SAFETY: The head index is private, so we can ensure it is always in bounds.
        let spawning_count = unsafe { *self.counts.get_unchecked(self.head) };
        self.head = (self.head + 1) % self.counts.len();

        // Set the last element of the ring to the popped value
        let tail_8 = (self.head + 8) % self.counts.len();
        self.counts[tail_8] = spawning_count;

        // Increment the 6th value of the ring by the popped value
        let tail_6 = (self.head + 6) % self.counts.len();
        self.counts[tail_6] += spawning_count;
    }

    fn step_n(&mut self, n: u64) {
        for _ in 0..n {
            self.step();
        }
    }

    fn total_count(&self) -> u64 {
        self.counts.iter().sum()
    }
}

fn input() -> FishState {
    let mut state = FishState {
        counts: [0; 9],
        head: 0,
    };
    for age in include_str!("../input.txt")
        .split(",")
        .map(|s| s.parse::<usize>().unwrap())
    {
        state.counts[age] += 1;
    }

    state
}

fn part_1(mut input: FishState) -> u64 {
    input.step_n(80);
    input.total_count()
}

fn part_2(mut input: FishState) -> u64 {
    input.step_n(256);
    input.total_count()
}

fn main() {
    let input = input();

    dbg!(part_1(input));
    dbg!(part_2(input));
}
