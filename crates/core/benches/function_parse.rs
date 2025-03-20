use std::time::{Duration, Instant};

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use rayon::prelude::*;

use parser_core::{parse, ParserError};
use parser_test_utils::read_test_file;

const TEST_FILESNAMES_BASE: &[&str] = &[
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

const TEST_FILESNAMES_IMAGES: &[&str] = &["test_jpg_1.jpg", "test_png_1.png", "test_webp_1.webp"];

const _TEST_FILESNAMES_FULL: &[&str] = &[
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
    // Create a vector of file data the size of the number of CPUs
    let file_data = read_test_file("test_pdf_1.pdf");
    let files: Vec<&[u8]> = vec![&file_data; num_cpus::get()];

    let mut group = c.benchmark_group("Sequential vs Parallel Parsing");

    group.throughput(Throughput::Elements(files.len() as u64));

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

fn benchmark_parallel_efficiency(c: &mut Criterion) {
    let file_data = read_test_file("test_pdf_1.pdf");

    let cpu_count = num_cpus::get();
    let mut counts = vec![
        cpu_count / 4,
        cpu_count / 2,
        cpu_count,
        cpu_count * 2,
        cpu_count * 4,
    ];
    counts.dedup();

    let mut group = c.benchmark_group("Parallel Efficiency");

    group.throughput(Throughput::Elements(1));

    for &count in &counts {
        let files: Vec<&[u8]> = vec![&file_data; count];

        group.bench_function(BenchmarkId::new("files", count), |b| {
            b.iter(|| {
                files
                    .par_iter()
                    .map(|d| parse(black_box(d)))
                    .collect::<Result<Vec<String>, ParserError>>()
            })
        });
    }

    group.finish();
}

fn benchmark_individual_files(c: &mut Criterion) {
    let cpus = num_cpus::get();
    let mut group = c.benchmark_group("Individual File Parsing");

    // Set throughput to the number of CPUs
    group.throughput(Throughput::Elements(cpus as u64));

    for &filename in TEST_FILESNAMES_BASE {
        let file = read_test_file(filename);
        let files: Vec<&[u8]> = vec![&file; cpus];

        group.bench_function(filename, |b| {
            b.iter(|| {
                files
                    .par_iter()
                    .map(|d| parse(black_box(d)))
                    .collect::<Result<Vec<String>, ParserError>>()
            })
        });
    }

    for &filename in TEST_FILESNAMES_IMAGES {
        let file = read_test_file(filename);
        let files: Vec<&[u8]> = vec![&file; cpus];

        group.sample_size(10);

        group.bench_function(filename, |b| {
            b.iter(|| {
                files
                    .par_iter()
                    .map(|d| parse(black_box(d)))
                    .collect::<Result<Vec<String>, ParserError>>()
            })
        });
    }

    group.finish();
}

// Finds the threshold number of files for each type that takes less than 16ms
fn benchmark_parallel_threshold(c: &mut Criterion) {
    let max_time_threshold = Duration::from_millis(16);

    // Read each test file only once
    for &filename in TEST_FILESNAMES_BASE {
        let file_extension = filename.split('.').last().unwrap_or("unknown");
        let group_name = format!("Parallel {} Processing", file_extension.to_uppercase());
        let mut group = c.benchmark_group(&group_name);

        // Cache the file data once
        let file_data = read_test_file(filename);

        // Function to measure processing time for a given count
        let measure_time = |count: usize| -> Duration {
            // Pre-allocate the vector of references outside timing
            let files: Vec<&[u8]> = vec![&file_data; count];

            // Perform warm-up runs to stabilize cache and runtime behavior
            for _ in 0..3 {
                black_box(
                    files
                        .par_iter()
                        .map(|d| parse(black_box(d)))
                        .collect::<Result<Vec<String>, ParserError>>()
                        .unwrap(),
                );
            }

            // Take multiple measurements and use median for robustness
            const SAMPLE_COUNT: usize = 5;
            let mut durations = Vec::with_capacity(SAMPLE_COUNT);

            for _ in 0..SAMPLE_COUNT {
                // Clear caches between runs to ensure consistent starting state
                black_box(());

                let start = Instant::now();
                black_box(
                    files
                        .par_iter()
                        .map(|d| parse(black_box(d)))
                        .collect::<Result<Vec<String>, ParserError>>()
                        .unwrap(),
                );
                durations.push(start.elapsed());
            }

            // Sort and take median duration (more robust against outliers)
            durations.sort();
            durations[SAMPLE_COUNT / 2]
        };

        // Finding and benchmarking the threshold count
        let mut low = 1;
        let mut high = 1;

        // Phase 1: Find upper bound using exponential search
        while measure_time(high) <= max_time_threshold {
            low = high;
            high *= 2;
        }

        // Phase 2: Binary search between bounds
        while high - low > 1 {
            let mid = low + (high - low) / 2;
            if measure_time(mid) <= max_time_threshold {
                low = mid;
            } else {
                high = mid;
            }
        }

        // The threshold count is now in 'low'
        let threshold_count = low;

        // Define percentages to test around the threshold
        let percentages = [90.0, 99.0, 99.9, 100.0, 100.1, 101.0, 110.0];

        // Generate test points based on percentages of the threshold
        let mut test_points: Vec<usize> = percentages
            .iter()
            .map(|&p| ((threshold_count as f64 * p / 100.0).ceil() as usize).max(1))
            .collect();

        test_points.dedup();

        // Benchmark each test point with proper throughput measurement
        for &count in &test_points {
            // Set throughput for proper operations/second measurements
            group.throughput(Throughput::Elements(count as u64));

            // Benchmark with the current count
            let files: Vec<&[u8]> = vec![&file_data; count];
            group.bench_with_input(BenchmarkId::new("files", count), &count, |b, &_| {
                b.iter(|| {
                    files
                        .par_iter()
                        .map(|d| parse(black_box(d)))
                        .collect::<Result<Vec<String>, ParserError>>()
                })
            });
        }

        // Add custom threshold marker to output
        println!(
            "Threshold for {}: {} files within {}ms",
            file_extension,
            threshold_count,
            max_time_threshold.as_millis()
        );

        group.finish();
    }
}

criterion_group!(
    benches,
    benchmark_sequential_vs_parallel,
    benchmark_parallel_efficiency,
    benchmark_individual_files,
    benchmark_parallel_threshold
);
criterion_main!(benches);
