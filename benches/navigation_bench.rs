// benches/navigation_bench.rs
use criterion::{criterion_group, criterion_main, Criterion};

fn bench_path_planning(c: &mut Criterion) {
    c.bench_function("a_star_100x100", |b| {
        b.iter(|| {
            let planner = AStarPlanner::new();
            planner.plan_path(Point::new(0, 0), Point::new(99, 99))
        })
    });
}

criterion_group!(benches, bench_path_planning);
criterion_main!(benches);
