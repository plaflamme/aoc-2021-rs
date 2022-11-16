use aoc_2021::*;
use aoc_lib::*;
use aocf::Aoc;
use criterion::{criterion_group, criterion_main, BatchSize, Criterion};

fn bench<D, A>(c: &mut Criterion, alt: A)
where
    D: Solver<A>,
    D: Day,
    A: std::fmt::Debug,
{
    let day = <D as Day>::DAY;
    let mut aoc = Aoc::new()
        .parse_cli(false)
        .year(Some(2021))
        .day(Some(day as u32))
        .init()
        .expect("unable to initialize Aoc");

    let input = aoc.get_input(false).expect("cannot read input");
    c.bench_function(format!("d{}-p1-{:?}", day, alt).as_str(), |b| {
        b.iter_batched(
            || <D as Solver<A>>::parse(input.as_str()),
            |input| <D as Solver<A>>::part1(input),
            BatchSize::SmallInput,
        );
    });
    c.bench_function(format!("d{}-p2-{:?}", day, alt).as_str(), |b| {
        b.iter_batched(
            || <D as Solver<A>>::parse(input.as_str()),
            |input| <D as Solver<A>>::part2(input),
            BatchSize::SmallInput,
        );
    });
}

macro_rules! bench_day {
    ($n: ident, $d: path) => {
        bench_day!($n, $d, Main);
    };
    ($n: ident, $d: path, $($alt: path),+) => {
        #[allow(non_snake_case)]
        fn $n(c: &mut Criterion) {
            $(bench::<$d, $alt>(c, $alt));+
        }
    };
}

bench_day!(day1, day1::Day1);
bench_day!(day2, day2::Day2);
bench_day!(day3, day3::Day3);
bench_day!(day4, day4::Day4);
bench_day!(day5, day5::Day5);
bench_day!(day6, day6::Day6);
bench_day!(day7, day7::Day7);
bench_day!(day8, day8::Day8);
bench_day!(day9, day9::Day9);
bench_day!(day10, day10::Day10);
bench_day!(day11, day11::Day11);
bench_day!(day12, day12::Day12);
bench_day!(day13, day13::Day13);
bench_day!(day14, day14::Day14);
bench_day!(day15, day15::Day15, Main, day15::AStar, day15::Dijkstra);
bench_day!(day16, day16::Day16, Main, day16::Bitter);
bench_day!(day17, day17::Day17);
bench_day!(day18, day18::Day18);
bench_day!(day19, day19::Day19);
bench_day!(day20, day20::Day20);
bench_day!(day21, day21::Day21);
bench_day!(day22, day22::Day22);
bench_day!(day23, day23::Day23);

criterion_group!(
    benches, day1, day2, day3, day4, day5, day6, day7, day8, day9, day10, day11, day12, day13,
    day14, day15, day16, day17, day18, day19, day20, day21, day22, day23,
);

criterion_main!(benches);
