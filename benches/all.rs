use aoc2021::*;
use aocf::Aoc;
use criterion::{criterion_group, criterion_main, BatchSize, Criterion};

fn bench_day<S: Solver>(c: &mut Criterion) {
    let day = <S as Solver>::DAY;
    let mut aoc = Aoc::new()
        .parse_cli(false)
        .year(Some(2021))
        .day(Some(day as u32))
        .init()
        .expect("unable to initialize Aoc");

    let input = aoc.get_input(false).expect("cannot read input");
    c.bench_function(format!("d{}p1", <S as Solver>::DAY).as_str(), |b| {
        b.iter_batched(
            || <S as Solver>::parse(input.as_str()),
            |s| s.part1(),
            BatchSize::SmallInput,
        );
    });
    c.bench_function(format!("d{}p2", <S as Solver>::DAY).as_str(), |b| {
        b.iter_batched(
            || <S as Solver>::parse(input.as_str()),
            |s| s.part2(),
            BatchSize::SmallInput,
        );
    });
}

criterion_group!(
    benches,
    bench_day::<day1::Solution>,
    bench_day::<day2::Solution>,
    bench_day::<day3::Solution>,
    bench_day::<day4::Solution>,
    bench_day::<day5::Solution>,
    bench_day::<day6::Solution>,
);
criterion_main!(benches);
