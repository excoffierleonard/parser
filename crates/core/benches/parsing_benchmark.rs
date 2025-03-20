use std::time::{Duration, Instant};

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rayon::prelude::*;

use parser_core::{parse, ParserError};
use parser_test_utils::read_test_file;

const TEST_FILENAMES_WITHOUT_OCR: &[&str] = &[
    "test_csv_1.csv",
    "test_docx_1.docx",
    "test_json_1.json",
    "test_pdf_1.pdf",
    "test_pptx_1.pptx",
    "test_txt_1.txt",
    "test_xlsx_1.xlsx",
];

const TEST_FILENAMES_WITH_OCR: &[&str] = &[
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

fn benchmark_sequential_vs_parallel(c: &mut Criterion) {
    let mut group = c.benchmark_group("Sequential vs Parallel Parsing");

    let files: Vec<Vec<u8>> = TEST_FILENAMES_WITHOUT_OCR
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
    for &filename in TEST_FILENAMES_WITH_OCR {
        let file = read_test_file(filename);

        group.bench_function(filename, |b| b.iter(|| parse(black_box(&file))));
    }

    group.finish();
}

fn benchmark_parallel_threshold(c: &mut Criterion) {
    // Threshold is 1 frame at 60 FPS
    let max_time_threshold = Duration::from_millis(16);

    // Test each file type separately
    for &filename in TEST_FILENAMES_WITHOUT_OCR {
        let file_extension = filename.split('.').last().unwrap_or("unknown");
        let group_name = format!("Parallel {} Processing", file_extension.to_uppercase());
        let mut group = c.benchmark_group(&group_name);

        let mut count = 1;
        loop {
            // Build test files of the current type
            let files: Vec<Vec<u8>> = (0..count).map(|_| read_test_file(filename)).collect();

            // Run a quick pre-benchmark to check duration
            let start = Instant::now();
            files
                .par_iter()
                .map(|d| parse(d))
                .collect::<Result<Vec<String>, ParserError>>()
                .unwrap();
            let duration = start.elapsed();

            // Only run the full benchmark if under threshold
            if duration < max_time_threshold {
                group.bench_function(format!("{} files", count), |b| {
                    b.iter(|| {
                        files
                            .par_iter()
                            .map(|d| parse(black_box(d)))
                            .collect::<Result<Vec<String>, ParserError>>()
                    })
                });
                count *= 2;
            } else {
                break;
            }
        }

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
