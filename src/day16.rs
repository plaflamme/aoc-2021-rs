use crate::{Day16, Solver};
use bitvec::prelude::*;
use itertools::Itertools;

type Bits = BitVec<Msb0, u8>;
type Slice = BitSlice<Msb0, u8>;

sample!(Day16, "A0016C880162017C3686B18A3D4780", "31");

#[derive(Debug, Clone, Eq, PartialEq)]
enum Op {
    Sum,
    Product,
    Min,
    Max,
    Gt,
    Lt,
    Equal,
}

impl From<u8> for Op {
    fn from(v: u8) -> Self {
        match v {
            0 => Op::Sum,
            1 => Op::Product,
            2 => Op::Min,
            3 => Op::Max,
            5 => Op::Gt,
            6 => Op::Lt,
            7 => Op::Equal,
            _ => panic!("invalid op {}", v),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum Token {
    Literal(usize),
    Op(Op, Vec<Packet>),
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Packet(u8, Token);

fn parse_varint(bits: &Slice) -> (usize, &Slice) {
    let mut lit = 0_usize;
    let mut rest = bits;
    loop {
        let (value, remain) = rest.split_at(5);
        let (first, value) = value.split_first().unwrap();
        lit <<= 4;
        lit |= value.load_be::<u8>() as usize;
        rest = remain;
        if !first {
            break;
        }
    }
    (lit, rest)
}

fn parse_literal(bits: &Slice) -> (Token, &Slice) {
    log::debug!("parse lit: {:?}", bits);
    let (lit, rest) = parse_varint(bits);

    (Token::Literal(lit), rest)
}

fn parse_op(op: u8, bits: &Slice) -> (Token, &Slice) {
    log::debug!("parse op {}: {:?}", op, bits);
    let (length_bit, rest) = bits.split_at(1);
    if *length_bit.get(0).unwrap() {
        // 11 bits
        let (n, bits) = rest.split_at(11);
        let n_packets: usize = n.load_be();

        let mut packets = Vec::new();
        let remain = (0..n_packets).fold(bits, |bits, _| {
            let (p, bits) = Packet::parse(bits);
            packets.push(p);
            bits
        });
        (Token::Op(Op::from(op), packets), remain)
    } else {
        // 15 bits
        let (n, bits) = rest.split_at(15);
        let length: usize = n.load_be();
        let (mut inner, bits) = bits.split_at(length);
        let mut packets = Vec::new();
        while !inner.is_empty() {
            let (packet, remain) = Packet::parse(inner);
            inner = remain;
            packets.push(packet);
        }
        (Token::Op(Op::from(op), packets), bits)
    }
}

impl Packet {
    fn parse(bits: &Slice) -> (Self, &Slice) {
        let (v, rest) = bits.split_at(3);
        let (tag, rest) = rest.split_at(3);

        let (token, rest) = match tag.load_be::<u8>() {
            4 => parse_literal(rest),
            op => parse_op(op, rest),
        };
        (Packet(v.load_be(), token), rest)
    }

    fn sum_versions(&self) -> usize {
        let inner_sum = match &self.1 {
            Token::Literal(_) => 0,
            Token::Op(_, packets) => packets.iter().map(|p| p.sum_versions()).sum(),
        };
        self.0 as usize + inner_sum
    }

    fn compute(self) -> usize {
        match self.1 {
            Token::Literal(v) => v,
            Token::Op(op, packets) => {
                let mut packets = packets.into_iter().map(|p| p.compute());
                match op {
                    Op::Sum => packets.sum(),
                    Op::Product => packets.product(),
                    Op::Min => packets.min().unwrap(),
                    Op::Max => packets.max().unwrap(),
                    _ => {
                        let (left, right) = packets.next_tuple().unwrap();
                        let result = match op {
                            Op::Lt => left < right,
                            Op::Gt => left > right,
                            Op::Equal => left == right,
                            _ => unreachable!(),
                        };
                        result.into()
                    }
                }
            }
        }
    }
}

impl Solver for Day16 {
    type Output = usize;

    type Input = Bits;

    fn parse(input: &str) -> Self::Input {
        input
            .trim()
            .chars()
            .map(|c| u8::from_str_radix(&c.to_string(), 16).unwrap())
            .tuples()
            .map(|(a, b)| Bits::from_element(a << 4 | b))
            .fold(Bits::new(), |mut v, b| {
                v.extend_from_bitslice(&b);
                v
            })
    }

    fn part1(input: Self::Input) -> Self::Output {
        Packet::parse(&input).0.sum_versions()
    }

    fn part2(input: Self::Input) -> Self::Output {
        Packet::parse(&input).0.compute()
    }
}

mod bitbit {

    use super::*;
    use bitter::BigEndianReader;
    use bitter::BitReader;

    fn parse_lit<B: BitReader>(reader: &mut B) -> Token {
        let mut lit = 0;
        loop {
            let more = reader.read_bit_unchecked();
            lit <<= 4;
            lit |= reader.read_bits_unchecked(4);
            if !more {
                break Token::Literal(lit as usize);
            }
        }
    }

    fn parse_op<B: BitReader>(op: Op, reader: &mut B) -> Token {
        if reader.read_bit_unchecked() {
            // 11 bits
            let n_packets = reader.read_bits_unchecked(11);
            let mut packets = Vec::with_capacity(n_packets as usize);
            (0..n_packets).for_each(|_| packets.push(parse_packet(reader)));
            Token::Op(op, packets)
        } else {
            // 15 bits
            let length = reader.read_bits_unchecked(15) as usize;
            let mut packets = Vec::new();
            let init = reader.bits_remaining().unwrap();
            loop {
                packets.push(parse_packet(reader));
                let remain = reader.bits_remaining().unwrap();
                if init - remain == length {
                    break;
                } else if init - remain > length {
                    panic!("oops")
                }
            }
            Token::Op(op, packets)
        }
    }

    fn parse_packet<B: BitReader>(reader: &mut B) -> Packet {
        let v = reader.read_bits_unchecked(3);
        let token = match reader.read_bits_unchecked(3) {
            4 => parse_lit(reader),
            op => parse_op(Op::from(op as u8), reader),
        };
        Packet(v as u8, token)
    }

    pub(super) fn parse(bits: &[u8]) -> Packet {
        parse_packet(&mut BigEndianReader::new(bits))
    }

    #[cfg(test)]
    mod test {
        use bitter::BigEndianReader;

        use super::*;

        #[test]
        fn test_parse_lit() {
            let bits = <Day16 as Solver<Bitter>>::parse("D2FE28");
            let lit = parse_packet(&mut BigEndianReader::new(&bits));
            assert_eq!(lit, Packet(6, Token::Literal(2021)));
        }

        #[test]
        fn test_parse_op() {
            let bits = <Day16 as Solver<Bitter>>::parse("38006F45291200");
            let packet = parse_packet(&mut BigEndianReader::new(&bits));
            assert_eq!(
                packet,
                Packet(
                    1,
                    Token::Op(
                        Op::from(6),
                        vec![Packet(6, Token::Literal(10)), Packet(2, Token::Literal(20))]
                    )
                )
            );
            let bits = <Day16 as Solver<Bitter>>::parse("EE00D40C823060");
            let packet = parse_packet(&mut BigEndianReader::new(&bits));
            assert_eq!(
                packet,
                Packet(
                    7,
                    Token::Op(
                        Op::from(3),
                        vec![
                            Packet(2, Token::Literal(1)),
                            Packet(4, Token::Literal(2)),
                            Packet(1, Token::Literal(3))
                        ]
                    )
                )
            );
        }
    }
}

#[derive(Debug)]
pub struct Bitter;
impl Solver<Bitter> for Day16 {
    type Output = usize;

    type Input = Vec<u8>;

    fn parse(input: &str) -> Self::Input {
        input
            .chars()
            .tuples()
            .map(|(a, b)| u8::from_str_radix(&String::from_iter([a, b]), 16).unwrap())
            .collect()
    }

    fn part1(input: Self::Input) -> Self::Output {
        bitbit::parse(&input).sum_versions()
    }

    fn part2(input: Self::Input) -> Self::Output {
        bitbit::parse(&input).compute()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_literal() {
        let bits = <Day16 as Solver>::parse("D2FE28");
        let (packet, _) = Packet::parse(&bits);
        assert_eq!(packet, Packet(6, Token::Literal(2021)));
    }
    #[test]
    fn test_parse_op() {
        let bits = <Day16 as Solver>::parse("38006F45291200");
        let (packet, _) = Packet::parse(&bits);
        assert_eq!(
            packet,
            Packet(
                1,
                Token::Op(
                    Op::from(6),
                    vec![Packet(6, Token::Literal(10)), Packet(2, Token::Literal(20))]
                )
            )
        );
        let bits = <Day16 as Solver>::parse("EE00D40C823060");
        let (packet, _) = Packet::parse(&bits);
        assert_eq!(
            packet,
            Packet(
                7,
                Token::Op(
                    Op::from(3),
                    vec![
                        Packet(2, Token::Literal(1)),
                        Packet(4, Token::Literal(2)),
                        Packet(1, Token::Literal(3))
                    ]
                )
            )
        );
    }
}
