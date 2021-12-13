use aoc2021::*;
use aocf::Aoc;
use criterion::{criterion_group, criterion_main, BatchSize, Criterion};

fn bench_main<D: Day>(c: &mut Criterion)
where
    D: Solver<Main>,
{
    bench_day::<D, Main>(c);
}
fn bench_day<D: Day, A>(c: &mut Criterion)
where
    D: Solver<A>,
{
    let day = <D as Day>::DAY;
    let mut aoc = Aoc::new()
        .parse_cli(false)
        .year(Some(2021))
        .day(Some(day as u32))
        .init()
        .expect("unable to initialize Aoc");

    let input = aoc.get_input(false).expect("cannot read input");
    c.bench_function(format!("d{}p1", day).as_str(), |b| {
        b.iter_batched(
            || <D as Solver<A>>::parse(input.as_str()),
            |input| <D as Solver<A>>::part1(input),
            BatchSize::SmallInput,
        );
    });
    c.bench_function(format!("d{}p2", day).as_str(), |b| {
        b.iter_batched(
            || <D as Solver<A>>::parse(input.as_str()),
            |input| <D as Solver<A>>::part2(input),
            BatchSize::SmallInput,
        );
    });
}

criterion_group!(
    benches,
    bench_main::<Day1>,
    bench_main::<Day2>,
    bench_main::<Day3>,
    bench_main::<Day4>,
    bench_main::<Day5>,
    bench_main::<Day6>,
    bench_main::<Day7>,
    bench_main::<Day8>,
    bench_main::<Day9>,
    bench_main::<Day10>,
    bench_main::<Day11>,
    bench_main::<Day12>,
    bench_main::<Day13>,
);
criterion_main!(benches);
