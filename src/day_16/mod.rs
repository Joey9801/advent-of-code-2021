struct BitIterator {
    nibbles: Vec<u8>,

    /// The index into `nibbles` that the next bit should come from the next bit should come from
    nibble_offset: usize,

    /// The offset into the current nibble that the next bit should come from
    ///
    /// Should always be 0, 1, 2, or 3,
    bit_offset: u8,
}

impl BitIterator {
    fn from_hex_str(s: &str) -> Self {
        let nibbles = s
            .bytes()
            .map(|b| match b {
                b'0'..=b'9' => b - b'0',
                b'A'..=b'F' => b - b'A' + 10,
                _ => panic!("Invalid byte"),
            })
            .collect();

        Self {
            nibbles,
            nibble_offset: 0,
            bit_offset: 0,
        }
    }

    fn take_n(&mut self, mut n: u8) -> u64 {
        let mut out = 0u64;
        while n > 0 {
            let bit_offset = 3 - self.bit_offset;
            let mask = 1 << bit_offset;
            let nibble = self.nibbles[self.nibble_offset];

            // The lsb contains a 1 iff this bit is set.
            let bit = ((nibble & mask) >> bit_offset) as u64;

            out |= bit << (n - 1);

            n -= 1;
            self.bit_offset += 1;
            self.nibble_offset += self.bit_offset as usize / 4;
            self.bit_offset %= 4;
        }

        out
    }

    fn remaining(&self) -> usize {
        (self.nibbles.len() - self.nibble_offset) * 4 + (4 - self.bit_offset) as usize
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct PacketId(usize);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ComparisonOp {
    GreaterThan,
    LessThan,
    EqualTo,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Operation {
    Sum,
    Product,
    Min,
    Max,
    Comparison(ComparisonOp),
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum PacketPayload {
    Literal {
        value: u64,
    },
    Operator {
        operation: Operation,
        sub_packets: Vec<PacketId>,
    },
}

#[derive(Clone, Debug)]
struct Packet {
    version: u8,
    payload: PacketPayload,
}

#[derive(Clone, Debug)]
pub struct Message {
    packets: Vec<Packet>,
    root: PacketId,
}

impl Message {
    fn next_packet_id(&self) -> PacketId {
        PacketId(self.packets.len())
    }

    fn eval(&self, packet: PacketId) -> u64 {
        match &self.packets[packet.0].payload {
            PacketPayload::Literal { value } => *value,
            PacketPayload::Operator {
                operation,
                sub_packets,
            } => {
                let mut sub_values = sub_packets.iter().map(|p| self.eval(*p));

                match operation {
                    Operation::Sum => sub_values.sum(),
                    Operation::Product => sub_values.product(),
                    Operation::Min => sub_values.min().unwrap(),
                    Operation::Max => sub_values.max().unwrap(),
                    Operation::Comparison(comp) => {
                        let a = sub_values.next().unwrap();
                        let b = sub_values.next().unwrap();
                        let pass = match comp {
                            ComparisonOp::GreaterThan => a > b,
                            ComparisonOp::LessThan => a < b,
                            ComparisonOp::EqualTo => a == b,
                        };

                        if pass {
                            1
                        } else {
                            0
                        }
                    }
                }
            }
        }
    }
}

fn parse_variable_length_integer(bits: &mut BitIterator) -> u64 {
    let mut out = 0u64;
    const CONTINUE_MASK: u64 = 0b10000;
    const PAYLOAD_MASK: u64 = 0b01111;
    loop {
        let chunk = bits.take_n(5);

        out = out << 4 | (chunk & PAYLOAD_MASK);
        if chunk & CONTINUE_MASK == 0 {
            break out;
        }
    }
}

fn parse_operator_payload(bits: &mut BitIterator, msg: &mut Message, id: u8) -> PacketPayload {
    let sub_packets = match bits.take_n(1) {
        0 => {
            let bit_count = bits.take_n(15);
            let target_remaining = bits.remaining() - bit_count as usize;
            let mut contents = Vec::new();
            while bits.remaining() > target_remaining {
                contents.push(parse_packet(bits, msg));
            }
            contents
        }
        1 => (0..bits.take_n(11))
            .map(|_| parse_packet(bits, msg))
            .collect(),
        _ => unreachable!(),
    };

    let operation = match id {
        0 => Operation::Sum,
        1 => Operation::Product,
        2 => Operation::Min,
        3 => Operation::Max,
        5 => Operation::Comparison(ComparisonOp::GreaterThan),
        6 => Operation::Comparison(ComparisonOp::LessThan),
        7 => Operation::Comparison(ComparisonOp::EqualTo),
        _ => panic!("Invalid operation ID"),
    };

    PacketPayload::Operator {
        operation,
        sub_packets,
    }
}

fn parse_packet(bits: &mut BitIterator, msg: &mut Message) -> PacketId {
    let version = bits.take_n(3) as u8;

    let payload = match bits.take_n(3) {
        4 => PacketPayload::Literal {
            value: parse_variable_length_integer(bits),
        },
        other => parse_operator_payload(bits, msg, other as u8),
    };

    let id = msg.next_packet_id();

    msg.packets.push(Packet { version, payload });
    id
}

fn parse_input(raw: &str) -> Message {
    let mut bits = BitIterator::from_hex_str(raw);
    let mut msg = Message {
        packets: Vec::new(),
        root: PacketId(0),
    };

    msg.root = parse_packet(&mut bits, &mut msg);

    msg
}

fn part_1(input: &Message) -> u32 {
    input.packets.iter().map(|p| p.version as u32).sum()
}

fn part_2(input: &Message) -> u64 {
    input.eval(input.root)
}

impl_day!("2021", "16", "Packet Decoder", Message, u32, u64);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bit_iterator() {
        let mut b = BitIterator::from_hex_str("D2FE28");
        assert_eq!(b.take_n(3), 0b110);
        assert_eq!(b.take_n(3), 0b100);
        assert_eq!(b.take_n(5), 0b10111);
        assert_eq!(b.take_n(5), 0b11110);
        assert_eq!(b.take_n(5), 0b00101);
    }

    #[test]
    fn test_eval() {
        // 1 + 2
        let msg = parse_input("C200B40A82");
        assert_eq!(msg.eval(msg.root), 3);

        // 6 * 9
        let msg = parse_input("04005AC33890");
        assert_eq!(msg.eval(msg.root), 54);

        // max(7, 8, 9)
        let msg = parse_input("CE00C43D881120");
        assert_eq!(msg.eval(msg.root), 9);

        // 5 > 15
        let msg = parse_input("D8005AC2A8F0");
        assert_eq!(msg.eval(msg.root), 1);

        // 5 < 15
        let msg = parse_input("F600BC2D8F");
        assert_eq!(msg.eval(msg.root), 0);

        // 5 == 15
        let msg = parse_input("9C005AC2F8F0");
        assert_eq!(msg.eval(msg.root), 0);

        // 1 + 3 == 2 * 2
        let msg = parse_input("9C0141080250320F1802104A08");
        assert_eq!(msg.eval(msg.root), 1);
    }
}
