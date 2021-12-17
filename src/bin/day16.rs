#[derive(Debug)]
enum Error {
    EofError,
    InvalidTypeId(u64),
    InvalidLengthTypeId(u64),
}

struct BitStream {
    i: usize,
    bits: String,
}

impl BitStream {
    fn eat_num(&mut self, width: usize) -> Result<u64, Error> {
        if self.i + width > self.bits.len() {
            return Err(Error::EofError);
        }
        let s = &self.bits[self.i..self.i + width];
        self.i += width;
        Ok(u64::from_str_radix(s, 2).unwrap())
    }
}

impl std::str::FromStr for BitStream {
    type Err = std::num::ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        let bytes = (0..s.len())
            .map(|i| u8::from_str_radix(&s[i..i + 1], 16))
            .collect::<Result<Vec<u8>, _>>()?;
        let bits = bytes
            .iter()
            .flat_map(|x| [0b1000 & x, 0b0100 & x, 0b0010 & x, 0b0001 & x])
            .map(|bit| if bit > 0 { "1" } else { "0" })
            .collect();
        Ok(BitStream { bits, i: 0 })
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

struct PacketParser {
    bits: BitStream,
}

impl PacketParser {
    fn parse_literal(&mut self) -> Result<u64, Error> {
        let mut val = 0;
        let mut done = false;
        while !done {
            done = self.bits.eat_num(1)? == 0;
            val = (val << 4) | self.bits.eat_num(4)?;
        }
        Ok(val)
    }

    fn parse_packets_by_count(&mut self) -> Result<Vec<Packet>, Error> {
        let count = self.bits.eat_num(11)?;
        (0..count).map(|_| self.parse_packet()).collect()
    }

    fn parse_packets_by_len(&mut self) -> Result<Vec<Packet>, Error> {
        let len = self.bits.eat_num(15)? as usize;
        let start = self.bits.i;
        let mut packets = Vec::new();
        while self.bits.i < start + len {
            packets.push(self.parse_packet()?);
        }
        Ok(packets)
    }

    fn parse_packets(&mut self) -> Result<Vec<Packet>, Error> {
        let length_type_id = self.bits.eat_num(1)?;
        match length_type_id {
            0 => self.parse_packets_by_len(),
            1 => self.parse_packets_by_count(),
            n => Err(Error::InvalidLengthTypeId(n)),
        }
    }

    fn parse_packet(&mut self) -> Result<Packet, Error> {
        let version = self.bits.eat_num(3)?;
        let type_id = self.bits.eat_num(3)?;
        let payload = match type_id {
            0 => Payload::Add(self.parse_packets()?),
            1 => Payload::Mul(self.parse_packets()?),
            2 => Payload::Min(self.parse_packets()?),
            3 => Payload::Max(self.parse_packets()?),
            4 => Payload::Literal(self.parse_literal()?),
            5 => Payload::Gt(self.parse_packets()?),
            6 => Payload::Lt(self.parse_packets()?),
            7 => Payload::Eq(self.parse_packets()?),
            n => return Err(Error::InvalidTypeId(n)),
        };
        Ok(Packet { version, payload })
    }
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

fn parse(bits: BitStream) -> Result<Packet, Error> {
    let mut parser = PacketParser { bits };
    parser.parse_packet()
}

fn main() {
    let path = std::env::args().nth(1).expect("missing input path");
    let text = std::fs::read_to_string(&path).unwrap();
    let bits = text.parse().unwrap();
    let packet = parse(bits).unwrap();
    println!("{}", version_sum(&packet));
    println!("{}", eval(&packet));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        for (result, input) in &[
            (16, "8A004A801A8002F478"),
            (12, "620080001611562C8802118E34"),
            (23, "C0015000016115A2E0802F182340"),
            (31, "A0016C880162017C3686B18A3D4780"),
        ] {
            let bits = input.parse().unwrap();
            let packet = parse(bits).unwrap();
            assert_eq!(*result, version_sum(&packet));
        }
    }

    #[test]
    fn test_part2() {
        for (result, input) in &[
            (3, "C200B40A82"),
            (54, "04005AC33890"),
            (7, "880086C3E88112"),
            (9, "CE00C43D881120"),
            (1, "D8005AC2A8F0"),
            (0, "F600BC2D8F"),
            (0, "9C005AC2F8F0"),
            (1, "9C0141080250320F1802104A08"),
        ] {
            let bits = input.parse().unwrap();
            let packet = parse(bits).unwrap();
            assert_eq!(*result, eval(&packet));
        }
    }
}
