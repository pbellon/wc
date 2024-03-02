use std::io;
use wc::{bytes_count, lines_count, words_count};

fn data_file(paragraphs: u32) -> String {
    format!("./scripts//benchmark/data/{}.txt", paragraphs)
}

fn test_lines(paragraphs: u32, expected: usize) -> io::Result<()> {
    let path = data_file(paragraphs);
    let lines = lines_count(&path)?;
    assert_eq!(lines, expected);
    Ok(())
}

fn test_words(paragraphs: u32, expected: usize) -> io::Result<()> {
    let path = data_file(paragraphs);
    let lines = words_count(&path)?;
    assert_eq!(lines, expected);
    Ok(())
}

fn test_bytes(paragraphs: u32, expected: usize) -> io::Result<()> {
    let path = data_file(paragraphs);
    let lines = bytes_count(&path)?;
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
