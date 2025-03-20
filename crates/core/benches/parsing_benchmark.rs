use std::time::{Duration, Instant};

use criterion::{black_box, criterion_group, criterion_main, BenchmarkGroup, Criterion};
use rayon::prelude::*;

use parser_core::{parse, ParserError};
use parser_test_utils::read_test_file;

const TEST_FILENAMES_NO_OCR: &[&str] = &[
    "test_csv_1.csv",
    "test_docx_1.docx",
    "test_json_1.json",
    "test_pdf_1.pdf",
    "test_pptx_1.pptx",
    "test_txt_1.txt",
    "test_xlsx_1.xlsx",
];

const TEST_FILENAMES: &[&str] = &[
    "test_csv_1.csv",
    "test_docx_1.docx",
    "test_jpg_1.jpg",
    "test_json_1.json",
    "test_pdf_1.pdf",
    "test_png_1.png",
    "test_pptx_1.pptx",
    "test_txt_1.txt",
    "test_webp_1.webp",
    "test_xlsx_1.xlsx",
];

const TEST_FILESNAMES_NO_TEXT_NO_OCR: &[&str] = &["test_pdf_1.pdf"];

fn benchmark_sequential_vs_parallel(c: &mut Criterion) {
    let mut group = c.benchmark_group("Sequential vs Parallel Parsing");

    let files: Vec<Vec<u8>> = TEST_FILENAMES_NO_OCR
        .iter()
        .map(|&filename| read_test_file(filename))
        .collect();

    // Benchmark parallel parsing
    group.bench_function("parallel", |b| {
        b.iter(|| {
            files
                .par_iter()
                .map(|d| parse(black_box(d)))
                .collect::<Result<Vec<String>, ParserError>>()
        })
    });

    // Benchmark sequential parsing
    group.bench_function("sequential", |b| {
        b.iter(|| {
            files
                .iter()
                .map(|d| parse(black_box(d)))
                .collect::<Result<Vec<String>, ParserError>>()
        })
    });

    group.finish();
}

fn benchmark_individual_files(c: &mut Criterion) {
    let mut group = c.benchmark_group("Individual File Parsing");

    // Benchmark parsing for individual files
    for &filename in TEST_FILENAMES {
        let file = read_test_file(filename);

        group.bench_function(filename, |b| b.iter(|| parse(black_box(&file))));
    }

    group.finish();
}

fn benchmark_parallel_threshold(c: &mut Criterion) {
    let max_time_threshold = Duration::from_millis(16);

    for &filename in TEST_FILESNAMES_NO_TEXT_NO_OCR {
        let file_extension = filename.split('.').last().unwrap_or("unknown");
        let group_name = format!("Parallel {} Processing", file_extension.to_uppercase());
        let mut group = c.benchmark_group(&group_name);

        // Closure to measure processing time for a given count
        let quick_test = |count: usize| -> Duration {
            let files: Vec<Vec<u8>> = (0..count).map(|_| read_test_file(filename)).collect();
            let start = Instant::now();
            files
                .par_iter()
                .map(|d| parse(d))
                .collect::<Result<Vec<String>, ParserError>>()
                .unwrap();
            start.elapsed()
        };

        // Closure to benchmark a specific count
        let benchmark_count = |group: &mut BenchmarkGroup<_>, count: usize| {
            let files: Vec<Vec<u8>> = (0..count).map(|_| read_test_file(filename)).collect();
            group.bench_function(format!("{} files", count), |b| {
                b.iter(|| {
                    files
                        .par_iter()
                        .map(|d| parse(black_box(d)))
                        .collect::<Result<Vec<String>, ParserError>>()
                })
            });
        };

        // Phase 1: Exponential search to find upper bound
        let mut count = 1;
        let mut last_good_count = 0;
        loop {
            let duration = quick_test(count);
            if duration > max_time_threshold {
                break;
            }
            benchmark_count(&mut group, count);
            last_good_count = count;
            count *= 2;
        }

        // Phase 2: Binary search to find exact threshold
        let mut low = last_good_count;
        let mut high = count;
        println!("Binary searching between {} and {} files", low, high);
        while high - low > 1 {
            let mid = low + (high - low) / 2;
            println!("Trying {} files", mid);
            let duration = quick_test(mid);
            println!("  Duration: {:?}", duration);
            if duration <= max_time_threshold {
                low = mid;
                benchmark_count(&mut group, mid);
                println!("  Updated low = {}", low);
            } else {
                high = mid;
                println!("  Updated high = {}", high);
            }
        }

        println!("Final threshold: {} files", low);
        group.finish();
    }
}

criterion_group!(
    benches,
    //benchmark_sequential_vs_parallel,
    //benchmark_individual_files,
    benchmark_parallel_threshold
);
criterion_main!(benches);
