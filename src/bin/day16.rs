fn parse_bits(s: &str) -> Result<String, std::num::ParseIntError> {
    let s = s.trim();
    let v = (0..s.len())
        .map(|i| u8::from_str_radix(&s[i..i + 1], 16))
        .collect::<Result<Vec<u8>, _>>()?;
    Ok(v.iter()
        .flat_map(|x| [0b1000 & x, 0b0100 & x, 0b0010 & x, 0b0001 & x])
        .map(|bit| if bit > 0 { "1" } else { "0" })
        .collect())
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
    Add(Vec<Packet>),
    Mul(Vec<Packet>),
    Min(Vec<Packet>),
    Max(Vec<Packet>),
    Gt(Vec<Packet>),
    Lt(Vec<Packet>),
    Eq(Vec<Packet>),
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

#[derive(Debug)]
enum Error {
    InvalidTypeId(u64),
    InvalidLengthTypeId(u64),
}

fn parse_packets_by_count(
    count: u64,
    bits: &mut BitStream,
) -> Result<Vec<Packet>, Error> {
    (0..count).map(|_| parse_packet(bits)).collect()
}

fn parse_packets_by_len(
    len: usize,
    bits: &mut BitStream,
) -> Result<Vec<Packet>, Error> {
    let start = bits.i;
    let mut packets = Vec::new();
    while bits.i < start + len {
        packets.push(parse_packet(bits)?);
    }
    Ok(packets)
}

fn parse_packets(bits: &mut BitStream) -> Result<Vec<Packet>, Error> {
    match bits.read_num(1) {
        0 => parse_packets_by_len(bits.read_num(15) as usize, bits),
        1 => parse_packets_by_count(bits.read_num(11), bits),
        n => Err(Error::InvalidLengthTypeId(n)),
    }
}

fn parse_packet(bits: &mut BitStream) -> Result<Packet, Error> {
    let version = bits.read_num(3);
    let type_id = bits.read_num(3);
    let payload = match type_id {
        0 => Payload::Add(parse_packets(bits)?),
        1 => Payload::Mul(parse_packets(bits)?),
        2 => Payload::Min(parse_packets(bits)?),
        3 => Payload::Max(parse_packets(bits)?),
        4 => Payload::Literal(parse_literal(bits)),
        5 => Payload::Gt(parse_packets(bits)?),
        6 => Payload::Lt(parse_packets(bits)?),
        7 => Payload::Eq(parse_packets(bits)?),
        n => return Err(Error::InvalidTypeId(n)),
    };
    Ok(Packet { version, payload })
}

fn version_sum(packet: &Packet) -> u64 {
    let sub_sum = match &packet.payload {
        Payload::Literal(_) => 0,
        Payload::Add(packets)
        | Payload::Mul(packets)
        | Payload::Max(packets)
        | Payload::Min(packets)
        | Payload::Gt(packets)
        | Payload::Lt(packets)
        | Payload::Eq(packets) => packets.iter().map(version_sum).sum(),
    };
    sub_sum + packet.version
}

fn eval(packet: &Packet) -> u64 {
    match &packet.payload {
        Payload::Literal(value) => *value,
        Payload::Add(packets) => packets.iter().map(eval).sum(),
        Payload::Mul(packets) => packets.iter().map(eval).product(),
        Payload::Max(packets) => packets.iter().map(eval).max().unwrap(),
        Payload::Min(packets) => packets.iter().map(eval).min().unwrap(),
        Payload::Gt(packets) => (eval(&packets[0]) > eval(&packets[1])) as u64,
        Payload::Lt(packets) => (eval(&packets[0]) < eval(&packets[1])) as u64,
        Payload::Eq(packets) => (eval(&packets[0]) == eval(&packets[1])) as u64,
    }
}

fn main() {
    let path = std::env::args().nth(1).expect("missing input path");
    let text = std::fs::read_to_string(&path).unwrap();
    let bits = parse_bits(&text).unwrap();
    let packet = parse_packet(&mut BitStream::new(bits)).unwrap();
    println!("{}", version_sum(&packet));
    println!("{}", eval(&packet));
}
