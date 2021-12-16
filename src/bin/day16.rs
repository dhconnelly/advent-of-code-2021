fn parse_bits(s: &str) -> String {
    let s = s.trim();
    (0..s.len())
        .map(|i| u8::from_str_radix(&s[i..i + 1], 16).unwrap())
        .flat_map(|x| {
            [0b1000 & x, 0b0100 & x, 0b0010 & x, 0b0001 & x].into_iter()
        })
        .map(|bit| if bit > 0 { "1" } else { "0" })
        .collect()
}

struct BitStream {
    i: usize,
    bits: String,
}

impl BitStream {
    fn new(bits: String) -> Self {
        Self { bits, i: 0 }
    }

    fn read_num(&mut self, width: usize) -> u64 {
        assert!(self.i + width <= self.bits.len());
        let s = &self.bits[self.i..self.i + width];
        self.i += width;
        u64::from_str_radix(s, 2).unwrap()
    }
}

#[derive(Debug)]
struct Packet {
    version: u64,
    payload: Payload,
}

#[derive(Debug)]
enum Payload {
    Literal(u64),
    Operation(Vec<Packet>),
}

fn parse_literal(bits: &mut BitStream) -> u64 {
    let mut val = 0;
    let mut done = false;
    while !done {
        done = bits.read_num(1) == 0;
        val = (val << 4) | bits.read_num(4);
    }
    val
}

fn parse_packets_count(count: u64, bits: &mut BitStream) -> Vec<Packet> {
    (0..count).map(|_| parse_packet(bits)).collect()
}

fn parse_packets_len(len: usize, bits: &mut BitStream) -> Vec<Packet> {
    let start = bits.i;
    let mut packets = Vec::new();
    while bits.i < start + len {
        packets.push(parse_packet(bits));
    }
    packets
}

fn parse_op(bits: &mut BitStream) -> Vec<Packet> {
    match bits.read_num(1) {
        0 => parse_packets_len(bits.read_num(15) as usize, bits),
        1 => parse_packets_count(bits.read_num(11), bits),
        _ => panic!(),
    }
}

fn parse_packet(bits: &mut BitStream) -> Packet {
    let version = bits.read_num(3);
    let type_id = bits.read_num(3);
    let payload = match type_id {
        4 => Payload::Literal(parse_literal(bits)),
        _ => Payload::Operation(parse_op(bits)),
    };
    Packet { version, payload }
}

fn version_sum(packet: &Packet) -> u64 {
    packet.version
        + match &packet.payload {
            Payload::Literal(_) => 0,
            Payload::Operation(packets) => {
                packets.iter().map(version_sum).sum()
            }
        }
}

fn main() {
    let path = std::env::args().nth(1).expect("missing input path");
    let text = std::fs::read_to_string(&path).unwrap();
    let bits = parse_bits(&text);
    let packet = parse_packet(&mut BitStream::new(bits));
    println!("{}", version_sum(&packet));
}
