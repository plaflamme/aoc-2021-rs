use crate::{Day16, Solver};
use bitvec::prelude::*;
use itertools::Itertools;

type Bits = BitVec<Msb0, u8>;
type Slice = BitSlice<Msb0, u8>;

sample!(Day16, "A0016C880162017C3686B18A3D4780", "31");

#[derive(Debug, Clone, Eq, PartialEq)]
enum Token {
    Literal(usize),
    Op(u8, Vec<Packet>),
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Packet(u8, Token);

fn parse_varint(bits: &Slice) -> (usize, &Slice) {
    let mut lit = 0_usize;
    let mut rest = bits;
    loop {
        lit <<= 4;
        let (n, remain) = rest.split_at(5);
        lit |= n.get(1..5).unwrap().load_be::<u8>() as usize;
        rest = remain;

        if !n.get(0).unwrap() {
            break (lit, rest);
        }
    }
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
        (Token::Op(op, packets), remain)
    } else {
        // 15 bits
        let (n, bits) = rest.split_at(15);
        let length: usize = n.load_be();
        let (mut inner, bits) = bits.split_at(length);
        let mut packets = Vec::new();
        while inner.len() > 0 {
            let (packet, remain) = Packet::parse(inner);
            inner = remain;
            packets.push(packet);
        }
        (Token::Op(op, packets), bits)
    }
}

impl Packet {
    fn parse(bits: &Slice) -> (Self, &Slice) {
        log::debug!("parse packet: {:?}", bits);
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
            Token::Literal(_) => 0_usize,
            Token::Op(_, packets) => packets.into_iter().map(|p| p.sum_versions()).sum::<usize>(),
        };
        self.0 as usize + inner_sum
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
        todo!()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_literal() {
        let bits = <Day16 as Solver>::parse("D2FE28");
        let (packet, rest) = Packet::parse(&bits);
        assert_eq!(packet, Packet(6, Token::Literal(2021)));
    }
    #[test]
    fn test_parse_op() {
        let bits = <Day16 as Solver>::parse("38006F45291200");
        let (packet, rest) = Packet::parse(&bits);
        assert_eq!(
            packet,
            Packet(
                1,
                Token::Op(
                    6,
                    vec![Packet(6, Token::Literal(10)), Packet(2, Token::Literal(20))]
                )
            )
        );
        let bits = <Day16 as Solver>::parse("EE00D40C823060");
        let (packet, rest) = Packet::parse(&bits);
        assert_eq!(
            packet,
            Packet(
                7,
                Token::Op(
                    3,
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
