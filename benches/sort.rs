use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use fix_float::*;
use rand::prelude::SliceRandom;
use rand::{self, SeedableRng};

const LOG_SCALE: &[f64] = &[1.0, 2.0, 5.0, 7.5];

fn log_scale(index: usize) -> usize {
    let mut res = 10usize.pow((index / LOG_SCALE.len()) as u32);

    res = ((res as f64) * LOG_SCALE[index % LOG_SCALE.len()]) as usize;
    res
}

fn bench_approaches(c: &mut Criterion) {
    let mut group = c.benchmark_group("sort approaches");
    let mut index = 0;

    loop {
        let n_len = log_scale(index);
        index += 1;
        if n_len > 10000 {
            break;
        }
        let v: Vec<f64> = (1..).take(n_len).map(|_| rand::random::<f64>()).collect();
        let vfix: Vec<ff64> = v.iter().map(|&x| x.try_into().unwrap()).collect();

        group.bench_with_input(BenchmarkId::new("total cmp", n_len), &n_len, |b, _| {
            b.iter_with_setup(
                || {
                    let mut v = v.clone();
                    v.shuffle(&mut rand_chacha::ChaCha8Rng::seed_from_u64(42));
                    v
                },
                |mut v| v.sort_by(|a, b| a.total_cmp(b)),
            );
        });

        group.bench_with_input(
            BenchmarkId::new("partial cmp then unwrap", n_len),
            &n_len,
            |b, _| {
                b.iter_with_setup(
                    || {
                        let mut v = v.clone();
                        v.shuffle(&mut rand_chacha::ChaCha8Rng::seed_from_u64(42));
                        v
                    },
                    |mut v| v.sort_by(|a, b| a.partial_cmp(b).unwrap()),
                );
            },
        );

        group.bench_with_input(
            BenchmarkId::new("fix floats sort", n_len),
            &n_len,
            |b, _| {
                b.iter_with_setup(
                    || {
                        let mut vfix = vfix.clone();
                        vfix.shuffle(&mut rand_chacha::ChaCha8Rng::seed_from_u64(42));
                        vfix
                    },
                    |mut vfix| vfix.sort(),
                );
            },
        );

        group.bench_with_input(
            BenchmarkId::new("fix floats sort_by", n_len),
            &n_len,
            |b, _| {
                b.iter_with_setup(
                    || {
                        let mut vfix = vfix.clone();
                        vfix.shuffle(&mut rand_chacha::ChaCha8Rng::seed_from_u64(42));
                        vfix
                    },
                    |mut vfix| vfix.sort_by(ff64::cmp),
                );
            },
        );
    }

    group.finish();
}

criterion_group!(benches, bench_approaches);
criterion_main!(benches);
