use std::{io, path::Path};
use wc::{line_count, words_count};

fn data_file(paragraphs: u32) -> String {
    format!("./scripts//benchmark/data/{}.txt", paragraphs)
}

fn test_lines(paragraphs: u32, expected: usize) -> io::Result<()> {
    let path_str = data_file(paragraphs);
    let path = Path::new(&path_str).canonicalize()?; // Assuming the file exists in a `tests` directory
    let lines = line_count(&path)?;
    assert_eq!(lines, expected);
    Ok(())
}

fn test_words(paragraphs: u32, expected: usize) -> io::Result<()> {
    let path_str = data_file(paragraphs);
    let path = Path::new(&path_str).canonicalize()?; // Assuming the file exists in a `tests` directory
    let lines = words_count(&path)?;
    assert_eq!(lines, expected);
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
fn test_word_count() -> io::Result<()> {
    test_words(100, 11444)?;
    test_words(200, 22253)?;
    test_words(500, 56390)?;
    Ok(())
}
