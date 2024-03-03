use std::io;
use wc::{count_file, process_files, CliParser, Metric, WcLineResult};

fn data_file(paragraphs: u32) -> String {
    format!("./scripts/benchmark/data/{}.txt", paragraphs)
}

fn test_lines(paragraphs: u32, expected: u64) -> io::Result<()> {
    let path = data_file(paragraphs);
    let lines = count_file(&path)?;
    assert_eq!(lines.0, expected);
    Ok(())
}

fn test_words(paragraphs: u32, expected: u64) -> io::Result<()> {
    let path = data_file(paragraphs);
    let lines = count_file(&path)?;
    assert_eq!(lines.1, expected);
    Ok(())
}

fn test_bytes(paragraphs: u32, expected: u64) -> io::Result<()> {
    let path = data_file(paragraphs);
    let lines = count_file(&path)?;
    assert_eq!(lines.2, expected);
    Ok(())
}

#[test]
fn test_line_count() -> io::Result<()> {
    test_lines(100, 199)?;
    test_lines(200, 399)?;
    test_lines(500, 999)?;
    Ok(())
}

#[test]
fn test_words_count() -> io::Result<()> {
    test_words(100, 11444)?;
    test_words(200, 22253)?;
    test_words(500, 56390)?;
    Ok(())
}

#[test]
fn test_bytes_count() -> io::Result<()> {
    test_bytes(100, 77159)?;
    test_bytes(200, 149861)?;
    test_bytes(500, 379590)?;
    Ok(())
}

#[test]
fn test_process_files_single() -> io::Result<()> {
    let paths = vec![data_file(100)];
    let metrics = vec![Metric::Lines, Metric::Words, Metric::Bytes];
    let results = process_files(&paths, &metrics)?;

    let expected: Vec<WcLineResult> = vec![WcLineResult {
        name: data_file(100),
        sizes: vec![199, 11444, 77159],
    }];

    assert_eq!(results, expected);

    Ok(())
}

#[test]
fn test_process_files_all() -> io::Result<()> {
    let paths = vec![data_file(100), data_file(200), data_file(500)];
    let metrics = vec![Metric::Lines, Metric::Words, Metric::Bytes];
    let results = process_files(&paths, &metrics)?;

    let expected: Vec<WcLineResult> = vec![
        WcLineResult {
            name: data_file(100),
            sizes: vec![199, 11444, 77159],
        },
        WcLineResult {
            name: data_file(200),
            sizes: vec![399, 22253, 149861],
        },
        WcLineResult {
            name: data_file(500),
            sizes: vec![999, 56390, 379590],
        },
        WcLineResult {
            name: "total".to_string(),
            sizes: vec![1597, 90087, 606610],
        },
    ];

    assert_eq!(results, expected);

    Ok(())
}

#[test]
fn test_process_files_partial_metrics() -> io::Result<()> {
    let paths = vec![data_file(100), data_file(200), data_file(500)];
    let metrics = vec![Metric::Words, Metric::Bytes];
    let results = process_files(&paths, &metrics)?;

    let expected: Vec<WcLineResult> = vec![
        WcLineResult {
            name: data_file(100),
            sizes: vec![11444, 77159],
        },
        WcLineResult {
            name: data_file(200),
            sizes: vec![22253, 149861],
        },
        WcLineResult {
            name: data_file(500),
            sizes: vec![56390, 379590],
        },
        WcLineResult {
            name: "total".to_string(),
            sizes: vec![90087, 606610],
        },
    ];

    assert_eq!(results, expected);

    Ok(())
}

#[test]
fn test_get_metrics_from_args_no_flags() {
    let args = CliParser {
        words: false,
        lines: false,
        bytes: false,
        files: vec![],
    };
    let metrics = args.get_metrics();
    let expected = vec![Metric::Lines, Metric::Words, Metric::Bytes];
    assert_eq!(metrics, expected);
}

#[test]
fn test_get_metrics_from_args_all_flags() {
    let args = CliParser {
        words: true,
        lines: true,
        bytes: true,
        files: vec![],
    };
    let metrics = args.get_metrics();
    let expected = vec![Metric::Lines, Metric::Words, Metric::Bytes];
    assert_eq!(metrics, expected);
}

#[test]
fn test_get_metrics_from_args_partial_flags() {
    let args = CliParser {
        words: true,
        lines: false,
        bytes: false,
        files: vec![],
    };
    let metrics = args.get_metrics();
    let expected = vec![Metric::Words];
    assert_eq!(metrics, expected);
}
