use criterion::{criterion_group, criterion_main, Criterion};
use parser_core::InputFiles;
use std::fs;
use std::path::Path;

fn load_test_files() -> Vec<Vec<u8>> {
    let test_files = [
        "tests/inputs/test_csv_1.csv",
        "tests/inputs/test_docx_1.docx",
        "tests/inputs/test_docx_2.docx",
        "tests/inputs/test_jpg_1.jpg",
        "tests/inputs/test_json_1.json",
        "tests/inputs/test_pdf_1.pdf",
        "tests/inputs/test_pdf_2.pdf",
        "tests/inputs/test_png_1.png",
        "tests/inputs/test_pptx_1.pptx",
        "tests/inputs/test_txt_1.txt",
        "tests/inputs/test_txt_2.txt",
        "tests/inputs/test_webp_1.webp",
        "tests/inputs/test_xlsx_1.xlsx",
        "tests/inputs/test_xlsx_2.xlsx",
    ];

    test_files
        .iter()
        .map(|&path| fs::read(Path::new(path)).expect("Failed to read test file"))
        .collect()
}

fn benchmark_parse_vs_sequential(c: &mut Criterion) {
    let files = load_test_files();

    let mut group = c.benchmark_group("Parsing Comparison");

    // Benchmark parallel parsing
    group.bench_function("parallel", |b| {
        b.iter(|| {
            let input_files = InputFiles::new(files.clone());
            input_files
                .parse()
                .expect("Failed to parse files in parallel")
        })
    });

    // Benchmark sequential parsing
    group.bench_function("sequential", |b| {
        b.iter(|| {
            let input_files = InputFiles::new(files.clone());
            input_files
                .parse_sequential()
                .expect("Failed to parse files sequentially")
        })
    });

    group.finish();
}

criterion_group!(benches, benchmark_parse_vs_sequential);
criterion_main!(benches);
