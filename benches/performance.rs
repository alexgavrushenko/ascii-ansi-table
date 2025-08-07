use ascii_ansi_table::{
    BorderUserConfig, ColumnUserConfig, TableUserConfig, table, utils::convert_ansi_to_html,
};
use criterion::{Criterion, black_box, criterion_group, criterion_main};

fn generate_test_data() -> Vec<Vec<String>> {
    let mut data = Vec::new();

    // Create deterministic test data matching pattern (a{1-5}-){50}
    let patterns = [
        "a-aa-aaa-aaaa-aaaaa-",
        "aa-aaa-aaaa-aaaaa-a-",
        "aaa-aaaa-aaaaa-a-aa-",
        "aaaa-aaaaa-a-aa-aaa-",
        "aaaaa-a-aa-aaa-aaaa-",
    ];

    for i in 0..10 {
        let mut row = Vec::new();
        for j in 0..3 {
            // Generate string matching pattern (a{1-5}-){50}
            let mut cell = String::new();
            for k in 0..50 {
                let pattern_idx = (i + j + k) % patterns.len();
                cell.push_str(patterns[pattern_idx]);
            }
            // Remove the last dash
            if cell.ends_with('-') {
                cell.pop();
            }
            row.push(cell);
        }
        data.push(row);
    }

    data
}

fn benchmark_table_wrapping(c: &mut Criterion) {
    let data = generate_test_data();

    // Create config with column width of 3
    let config = TableUserConfig {
        border: Some(BorderUserConfig {
            top_body: Some("─".to_string()),
            top_join: Some("┬".to_string()),
            top_left: Some("┌".to_string()),
            top_right: Some("┐".to_string()),
            bottom_body: Some("─".to_string()),
            bottom_join: Some("┴".to_string()),
            bottom_left: Some("└".to_string()),
            bottom_right: Some("┘".to_string()),
            body_left: Some("│".to_string()),
            body_right: Some("│".to_string()),
            body_join: Some("│".to_string()),
            header_join: Some("─".to_string()),
            join_body: Some("─".to_string()),
            join_left: Some("├".to_string()),
            join_right: Some("┤".to_string()),
            join_join: Some("┼".to_string()),
        }),
        columns: Some(vec![
            ColumnUserConfig {
                width: Some(3),
                wrap_word: Some(false), // Character wrapping for maximum stress
                alignment: None,
                vertical_alignment: None,
                padding_left: None,
                padding_right: None,
                truncate: None,
            };
            3
        ]),
        column_default: None,
        single_line: None,
        spanning_cells: None,
        header: None,
    };

    c.bench_function("table_wrapping_10x3_width3", |b| {
        b.iter(|| {
            let result = table(black_box(&data), black_box(Some(&config)));
            black_box(result)
        })
    });
}

fn benchmark_wrapping_components(c: &mut Criterion) {
    let data = generate_test_data();

    // Benchmark just the wrapping function
    let sample_text = &data[0][0];

    c.bench_function("wrap_text_width_3", |b| {
        b.iter(|| {
            ascii_ansi_table::wrap_text(black_box(sample_text), black_box(3), black_box(false))
        })
    });

    // Benchmark cell height calculation
    c.bench_function("calculate_cell_height_width_3", |b| {
        b.iter(|| {
            ascii_ansi_table::calculate_cell_height(
                black_box(sample_text),
                black_box(3),
                black_box(false),
            )
        })
    });
}

fn benchmark_ansi(c: &mut Criterion) {
    let data = generate_test_data();

    // Benchmark just the wrapping function
    let sample_text = ascii_ansi_table::wrap_text(&text, 3, false);

    c.bench_function("convert_ansi_to_html", |b| {
        b.iter(|| {
            convert_ansi_to_html(black_box(&sample_text.join("\n")));
        })
    });
}

fn benchmark_memory_usage(c: &mut Criterion) {
    let data = generate_test_data();

    let config = TableUserConfig {
        border: Some(BorderUserConfig {
            top_body: Some("─".to_string()),
            top_join: Some("┬".to_string()),
            top_left: Some("┌".to_string()),
            top_right: Some("┐".to_string()),
            bottom_body: Some("─".to_string()),
            bottom_join: Some("┴".to_string()),
            bottom_left: Some("└".to_string()),
            bottom_right: Some("┘".to_string()),
            body_left: Some("│".to_string()),
            body_right: Some("│".to_string()),
            body_join: Some("│".to_string()),
            header_join: Some("─".to_string()),
            join_body: Some("─".to_string()),
            join_left: Some("├".to_string()),
            join_right: Some("┤".to_string()),
            join_join: Some("┼".to_string()),
        }),
        columns: Some(vec![
            ColumnUserConfig {
                width: Some(3),
                wrap_word: Some(false),
                alignment: None,
                vertical_alignment: None,
                padding_left: None,
                padding_right: None,
                truncate: None,
            };
            3
        ]),
        column_default: None,
        single_line: None,
        spanning_cells: None,
        header: None,
    };

    c.bench_function("memory_efficient_rendering", |b| {
        b.iter(|| {
            let result =
                table(black_box(&data), black_box(Some(&config))).unwrap_or_else(|_| String::new());
            black_box(result)
        });
    });
}

criterion_group!(
    benches,
    benchmark_table_wrapping,
    benchmark_wrapping_components,
    benchmark_memory_usage,
    benchmark_ansi,
);
criterion_main!(benches);
