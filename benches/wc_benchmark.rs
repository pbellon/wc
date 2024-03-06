use criterion::{black_box, criterion_group, criterion_main, Criterion};
use wc::{count_file, process_files, Metric};

fn data_file(name: &str) -> String {
    format!("./scripts/benchmark/data/{}", name)
}

fn benchmark_count_file(c: &mut Criterion) {
    c.bench_function("count_file", |b| {
        b.iter(|| count_file(black_box(&data_file("900.txt"))))
    });
}

fn benchmark_process_files(c: &mut Criterion) {
    let paths = vec![
        data_file("100.txt"),
        data_file("200.txt"),
        data_file("500.txt"),
        data_file("700.txt"),
        data_file("900.txt"),
    ];
    let metrics = vec![Metric::Lines, Metric::Words, Metric::Bytes];

    c.bench_function("process_files", |b| {
        b.iter(|| process_files(&paths, &metrics))
    });
}

criterion_group!(benches, benchmark_count_file, benchmark_process_files);
criterion_main!(benches);
