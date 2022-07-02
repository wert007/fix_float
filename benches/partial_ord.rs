use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_approaches(c: &mut Criterion) {
    let mut group = c.benchmark_group("partial ord approaches");

    let setup = || {
        let a = rand::random::<f64>();
        let b = rand::random::<f64>();

        (a, b)
    };

    group.bench_function("partial_cmp", |b| {
        b.iter_with_setup(setup, |(a, b)| {
            black_box(a);
            black_box(b);
            black_box(a.partial_cmp(&b))
        });
    });

    group.bench_function("Some(total_cmp)", |b| {
        b.iter_with_setup(setup, |(a, b)| {
            black_box(a);
            black_box(b);
            black_box(Some(a.total_cmp(&b)))
        });
    });

    // old magic old spell from float_ord
    // coming from: http://stereopsis.com/radix.html
    group.bench_function("radix sort", |b| {
        b.iter_with_setup(setup, |(a, b)| {
            black_box(a);
            black_box(b);
            let a = a.to_bits();
            let b = b.to_bits();
            let sbit = 1 << 63;

            let a = if a & sbit != 0 { !a } else { a | sbit };
            let b = if b & sbit != 0 { !b } else { b | sbit };

            black_box(a.partial_cmp(&b))
        });
    });

    group.finish();
}

criterion_group!(benches, bench_approaches);
criterion_main!(benches);
