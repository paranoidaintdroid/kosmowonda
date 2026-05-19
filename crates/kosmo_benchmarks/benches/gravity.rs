use criterion::{Criterion, criterion_group, criterion_main};

use kosmo_math::Vec2;
use kosmo_physics::{Body, accumulate_forces, accumulate_forces_adaptive, accumulate_forces_parallel};

fn make_bodies(n: usize) -> Vec<Body> {
    (0..n)
        .map(|i| {
            let t = i as f64 * std::f64::consts::TAU / n as f64;

            Body::new(
                1.0,
                Vec2::new(t.cos() * 10.0, t.sin() * 10.0),
                Vec2::new(0.0, 0.0),
            )
        })
        .collect()
}

fn gravity_bench(c: &mut Criterion) {
    for n in [10, 100, 500] {
        let bodies = make_bodies(n);

        c.bench_function(&format!("seq_gravity_{n}"), |b| {
            b.iter(|| {
                let mut bodies = bodies.clone();

                accumulate_forces(&mut bodies);
            })
        });
    }
}

fn gravity_bench_parallel(c: &mut Criterion) {
    for n in [10, 100, 500] {
        let bodies = make_bodies(n);

        c.bench_function(&format!("par_gravity_{n}"), |b| {
            b.iter(|| {
                let mut bodies = bodies.clone();

                accumulate_forces_parallel(&mut bodies);
            })
        });
    }
}

fn gravity_bench_adaptive(c: &mut Criterion) {
    for n in [10, 100, 500] {
        let bodies = make_bodies(n);

        c.bench_function(&format!("adp_gravity_{n}"), |b| {
            b.iter(|| {
                let mut bodies = bodies.clone();

                accumulate_forces_adaptive(&mut bodies);
            })
        });
    }
}

criterion_group!(benches, gravity_bench, gravity_bench_parallel, gravity_bench_adaptive);

criterion_main!(benches);
