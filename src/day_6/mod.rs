#[derive(Clone, Copy)]
pub struct FishState {
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

fn parse_input(raw: &str) -> FishState {
    let mut state = FishState {
        counts: [0; 9],
        head: 0,
    };
    for age in raw
        .split(",")
        .map(|s| s.parse::<usize>().unwrap())
    {
        state.counts[age] += 1;
    }

    state
}

fn part_1(input: &FishState) -> u64 {
    let mut input = input.clone();
    input.step_n(80);
    input.total_count()
}

fn part_2(input: &FishState) -> u64 {
    let mut input = input.clone();
    input.step_n(256);
    input.total_count()
}

impl_day!("2021", "6", "Laternfish", FishState, u64, u64);