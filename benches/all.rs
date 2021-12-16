use aoc2021::*;
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
    ($d: ident) => {
        bench_day!($d, Main);
    };
    ($d: ident, $($alt: path),+) => {
        #[allow(non_snake_case)]
        fn $d(c: &mut Criterion) {
            $(bench::<$d, $alt>(c, $alt));+
        }
    };
}

bench_day!(Day1);
bench_day!(Day2);
bench_day!(Day3);
bench_day!(Day4);
bench_day!(Day5);
bench_day!(Day6);
bench_day!(Day7);
bench_day!(Day8);
bench_day!(Day9);
bench_day!(Day10);
bench_day!(Day11);
bench_day!(Day12);
bench_day!(Day13);
bench_day!(Day14);
bench_day!(Day15, Main, day15::AStar, day15::Dijkstra);

criterion_group!(
    benches, Day1, Day2, Day3, Day4, Day5, Day6, Day7, Day8, Day9, Day10, Day11, Day12, Day13,
    Day14, Day15
);
criterion_main!(benches);
