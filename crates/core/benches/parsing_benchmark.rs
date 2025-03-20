use criterion::{black_box, criterion_group, criterion_main, Criterion};
use parser_core::{parse, ParserError};
use parser_test_utils::read_test_file;
use rayon::prelude::*;

const TEST_FILENAMES_WITHOUT_OCR: &[&str] = &[
    "test_csv_1.csv",
    "test_docx_1.docx",
    "test_docx_2.docx",
    "test_json_1.json",
    "test_pdf_1.pdf",
    "test_pdf_2.pdf",
    "test_pptx_1.pptx",
    "test_txt_1.txt",
    "test_txt_2.txt",
    "test_xlsx_1.xlsx",
    "test_xlsx_2.xlsx",
];

const TEST_FILENAMES_WITH_OCR: &[&str] = &[
    "test_csv_1.csv",
    "test_docx_1.docx",
    "test_docx_2.docx",
    "test_jpg_1.jpg",
    "test_json_1.json",
    "test_pdf_1.pdf",
    "test_pdf_2.pdf",
    "test_png_1.png",
    "test_pptx_1.pptx",
    "test_txt_1.txt",
    "test_txt_2.txt",
    "test_webp_1.webp",
    "test_xlsx_1.xlsx",
    "test_xlsx_2.xlsx",
];

fn benchmark_sequential_vs_parallel(c: &mut Criterion) {
    let files: Vec<Vec<u8>> = TEST_FILENAMES_WITHOUT_OCR
        .iter()
        .map(|&filename| read_test_file(filename))
        .collect();

    let mut group = c.benchmark_group("Sequential vs Parallel Parsing");

    // Benchmark parallel parsing
    group.bench_function("parallel", |b| {
        b.iter(|| {
            files
                .par_iter()
                .map(|d| parse(black_box(d)))
                .collect::<Result<Vec<String>, ParserError>>()
                .unwrap()
        })
    });

    // Benchmark sequential parsing
    group.bench_function("sequential", |b| {
        b.iter(|| {
            files
                .iter()
                .map(|d| parse(black_box(d)))
                .collect::<Result<Vec<String>, ParserError>>()
                .unwrap()
        })
    });

    group.finish();
}

fn benchmark_individual_files(c: &mut Criterion) {
    let mut group = c.benchmark_group("Individual File Parsing");

    for &filename in TEST_FILENAMES_WITH_OCR {
        let file_data = read_test_file(filename);

        group.bench_function(filename, |b| {
            b.iter(|| parse(black_box(&file_data)).unwrap())
        });
    }

    group.finish();
}

criterion_group!(
    benches,
    benchmark_sequential_vs_parallel,
    benchmark_individual_files
);
criterion_main!(benches);
