use criterion::{criterion_group, criterion_main, Criterion};
use parser_core::parse;
use parser_test_utils::read_test_file;
use rayon::prelude::*;

fn get_test_filenames() -> Vec<&'static str> {
    vec![
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
    ]
}

fn load_test_files() -> Vec<Vec<u8>> {
    get_test_filenames()
        .iter()
        .map(|&filename| read_test_file(filename))
        .collect()
}

fn benchmark_sequential_vs_parallel(c: &mut Criterion) {
    let files = load_test_files();

    let mut group = c.benchmark_group("Parsing Comparison");

    // Benchmark parallel parsing
    group.bench_function("parallel", |b| {
        b.iter(|| {
            files
                .par_iter()
                .map(|d| parse(d))
                .collect::<Result<Vec<_>, _>>()
                .expect("Failed to parse files in parallel")
        })
    });

    // Benchmark sequential parsing
    group.bench_function("sequential", |b| {
        b.iter(|| {
            files
                .iter()
                .map(|d| parse(d))
                .collect::<Result<Vec<_>, _>>()
                .expect("Failed to parse files sequentially")
        })
    });

    group.finish();
}

fn benchmark_individual_files(c: &mut Criterion) {
    let filenames = get_test_filenames();
    let mut group = c.benchmark_group("Individual File Parsing");

    for (index, &filename) in filenames.iter().enumerate() {
        let file_data = read_test_file(filename);

        group.bench_function(filename, |b| {
            b.iter(|| parse(&file_data).expect(&format!("Failed to parse {}", filename)))
        });
    }

    group.finish();
}

criterion_group!(benches, benchmark_individual_files);
criterion_main!(benches);
