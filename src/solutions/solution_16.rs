use std::cmp::{max, min, Ordering};

pub fn a(input: &str) {
    let mut binary_string = make_binary_string(input);
    let packet = parse(&mut binary_string);
    println!("{}", packet.version_sum());
}

pub fn b(input: &str) {
    let mut binary_string = make_binary_string(input);
    let packet = parse(&mut binary_string);
    println!("{}", packet.eval());
}

enum Packet {
    Literal {
        version: usize,
        value: usize,
    },
    Operator {
        version: usize,
        id: usize,
        length_type_id: usize,
        sub_packets: Vec<Packet>,
    },
}

impl Packet {
    fn version_sum(&self) -> usize {
        match self {
            Packet::Literal { version, .. } => *version,
            Packet::Operator {
                version,
                sub_packets,
                ..
            } => *version + sub_packets.iter().map(|p| p.version_sum()).sum::<usize>(),
        }
    }

    fn eval(&self) -> usize {
        match self {
            Packet::Literal { value, .. } => *value,
            Packet::Operator {
                id, sub_packets, ..
            } => match *id {
                0 => sub_packets.iter().map(|p| p.eval()).sum::<usize>(),
                1 => sub_packets.iter().map(|p| p.eval()).product::<usize>(),
                2 => sub_packets.iter().map(|p| p.eval()).min().unwrap(),
                3 => sub_packets.iter().map(|p| p.eval()).max().unwrap(),
                5 | 6 | 7 => {
                    let a = sub_packets[0].eval();
                    let b = sub_packets[1].eval();
                    match a.cmp(&b) {
                        Ordering::Greater => {
                            if *id == 5 {
                                1
                            } else {
                                0
                            }
                        }
                        Ordering::Less => {
                            if *id == 6 {
                                1
                            } else {
                                0
                            }
                        }
                        Ordering::Equal => {
                            if *id == 7 {
                                1
                            } else {
                                0
                            }
                        }
                    }
                }
                _ => unreachable!(),
            },
        }
    }
}

fn make_binary_string(input: &str) -> Vec<u8> {
    input
        .trim_end()
        .chars()
        .flat_map(|c| hex_to_bin(&c).bytes().map(|b| b - b'0').collect::<Vec<_>>())
        .collect()
}

fn parse(packet: &mut Vec<u8>) -> Packet {
    let version = consume_bits(packet, 3);
    let type_id = consume_bits(packet, 3);
    match type_id {
        4 => {
            // Literal value. Read by groups of 5 bits. First bit of each group will be 1 until
            // final group (0)
            let mut x = !0; // 0xffff...f
            let mut value = 0;
            while x >> 4 != 0 {
                x = consume_bits(packet, 5);
                value = (value << 4) + (x & 0b1111);
            }
            Packet::Literal { version, value }
        }
        id => {
            // Operator packet
            let length_type_id = consume_bits(packet, 1);
            let sub_packets = match length_type_id {
                0 => {
                    // next 15 bits represent the tolal length in bits of the sub-packets contained
                    // within
                    let l = consume_bits(packet, 15);
                    let mut packet_bits = packet.drain(..l).collect::<Vec<u8>>();
                    let mut sub_packets = Vec::new();
                    while !packet_bits.is_empty() {
                        sub_packets.push(parse(&mut packet_bits));
                    }
                    sub_packets
                }
                1 => {
                    // next 11 bits represent the number of sub-packets contained within
                    let n = consume_bits(packet, 11);
                    let mut sub_packets = Vec::new();
                    for _ in 0..n {
                        sub_packets.push(parse(packet));
                    }
                    sub_packets
                }
                _ => unreachable!(),
            };
            Packet::Operator {
                version,
                id,
                length_type_id,
                sub_packets,
            }
        }
    }
}

fn consume_bits(bits: &mut Vec<u8>, n: usize) -> usize {
    let mut x = 0;
    for b in bits.drain(0..n) {
        // eg
        // x = ...0110 and b = 1
        // x<<1 => ...1100
        // 1100 | 1 = 1101
        x = (x << 1) | b as usize;
    }
    x
}

fn hex_to_bin(c: &char) -> &str {
    match c {
        '0' => "0000",
        '1' => "0001",
        '2' => "0010",
        '3' => "0011",
        '4' => "0100",
        '5' => "0101",
        '6' => "0110",
        '7' => "0111",
        '8' => "1000",
        '9' => "1001",
        'A' => "1010",
        'B' => "1011",
        'C' => "1100",
        'D' => "1101",
        'E' => "1110",
        'F' => "1111",
        _ => unreachable!(),
    }
}
