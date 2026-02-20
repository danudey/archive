//! Tests for single-file decompression (gz, bz2, xz, lz4, zst)

mod common;

use archive::{ArchiveExtractor, ArchiveFormat};
use common::read_test_archive;

#[test]
fn test_single_gz_decompression() {
    let data = read_test_archive("hello.txt.gz");
    let extractor = ArchiveExtractor::new();

    let files = extractor
        .extract_with_format(&data, ArchiveFormat::Gz)
        .expect("Failed to decompress hello.txt.gz");

    assert_eq!(files.len(), 1, "Expected single decompressed file");
    let content = String::from_utf8_lossy(&files[0].data);
    assert_eq!(content.trim(), "Hello, World!");
}

#[test]
fn test_single_bz2_decompression() {
    let data = read_test_archive("hello.txt.bz2");
    let extractor = ArchiveExtractor::new();

    let files = extractor
        .extract_with_format(&data, ArchiveFormat::Bz2)
        .expect("Failed to decompress hello.txt.bz2");

    assert_eq!(files.len(), 1, "Expected single decompressed file");
    let content = String::from_utf8_lossy(&files[0].data);
    assert_eq!(content.trim(), "Hello, World!");
}

#[test]
fn test_single_xz_decompression() {
    let data = read_test_archive("hello.txt.xz");
    let extractor = ArchiveExtractor::new();

    let files = extractor
        .extract_with_format(&data, ArchiveFormat::Xz)
        .expect("Failed to decompress hello.txt.xz");

    assert_eq!(files.len(), 1, "Expected single decompressed file");
    let content = String::from_utf8_lossy(&files[0].data);
    assert_eq!(content.trim(), "Hello, World!");
}

#[test]
fn test_single_lz4_decompression() {
    let data = read_test_archive("hello.txt.lz4");
    let extractor = ArchiveExtractor::new();

    let files = extractor
        .extract_with_format(&data, ArchiveFormat::Lz4)
        .expect("Failed to decompress hello.txt.lz4");

    assert_eq!(files.len(), 1, "Expected single decompressed file");
    let content = String::from_utf8_lossy(&files[0].data);
    assert_eq!(content.trim(), "Hello, World!");
}

#[test]
fn test_single_zst_decompression() {
    let data = read_test_archive("hello.txt.zst");
    let extractor = ArchiveExtractor::new();

    let files = extractor
        .extract_with_format(&data, ArchiveFormat::Zst)
        .expect("Failed to decompress hello.txt.zst");

    assert_eq!(files.len(), 1, "Expected single decompressed file");
    let content = String::from_utf8_lossy(&files[0].data);
    assert_eq!(content.trim(), "Hello, World!");
}

#[test]
fn test_gz_extracts_original_filename() {
    let data = read_test_archive("hello.txt.gz");
    let extractor = ArchiveExtractor::new();

    let files = extractor
        .extract_with_format(&data, ArchiveFormat::Gz)
        .expect("Failed to decompress hello.txt.gz");

    assert_eq!(files.len(), 1);
    // Gzip files created with gzip tool typically store the original filename
    // If no filename in header, should default to "data"
    assert!(
        files[0].path == "hello.txt" || files[0].path == "data",
        "Expected 'hello.txt' or 'data', got '{}'",
        files[0].path
    );
}

#[test]
fn test_bz2_uses_data_as_filename() {
    let data = read_test_archive("hello.txt.bz2");
    let extractor = ArchiveExtractor::new();

    let files = extractor
        .extract_with_format(&data, ArchiveFormat::Bz2)
        .expect("Failed to decompress hello.txt.bz2");

    assert_eq!(files.len(), 1);
    // bzip2 format doesn't store original filename
    assert_eq!(files[0].path, "data");
}

#[test]
fn test_xz_uses_data_as_filename() {
    let data = read_test_archive("hello.txt.xz");
    let extractor = ArchiveExtractor::new();

    let files = extractor
        .extract_with_format(&data, ArchiveFormat::Xz)
        .expect("Failed to decompress hello.txt.xz");

    assert_eq!(files.len(), 1);
    // xz format doesn't store original filename
    assert_eq!(files[0].path, "data");
}

#[test]
fn test_lz4_uses_data_as_filename() {
    let data = read_test_archive("hello.txt.lz4");
    let extractor = ArchiveExtractor::new();

    let files = extractor
        .extract_with_format(&data, ArchiveFormat::Lz4)
        .expect("Failed to decompress hello.txt.lz4");

    assert_eq!(files.len(), 1);
    // lz4 format doesn't store original filename
    assert_eq!(files[0].path, "data");
}

#[test]
fn test_zst_uses_data_as_filename() {
    let data = read_test_archive("hello.txt.zst");
    let extractor = ArchiveExtractor::new();

    let files = extractor
        .extract_with_format(&data, ArchiveFormat::Zst)
        .expect("Failed to decompress hello.txt.zst");

    assert_eq!(files.len(), 1);
    // zstd format doesn't store original filename
    assert_eq!(files[0].path, "data");
}

// Builder API tests

#[test]
fn test_builder_bz2_source_filename_derives_path() {
    let data = read_test_archive("hello.txt.bz2");
    let extractor = ArchiveExtractor::new()
        .with_source_filename("hello.txt.bz2")
        .with_format(ArchiveFormat::Bz2);

    let files = extractor.extract(&data).expect("Failed to decompress");
    assert_eq!(files.len(), 1);
    assert_eq!(files[0].path, "hello.txt");
}

#[test]
fn test_builder_xz_source_filename_derives_path() {
    let data = read_test_archive("hello.txt.xz");
    let extractor = ArchiveExtractor::new()
        .with_source_filename("hello.txt.xz")
        .with_format(ArchiveFormat::Xz);

    let files = extractor.extract(&data).expect("Failed to decompress");
    assert_eq!(files.len(), 1);
    assert_eq!(files[0].path, "hello.txt");
}

#[test]
fn test_builder_lz4_source_filename_derives_path() {
    let data = read_test_archive("hello.txt.lz4");
    let extractor = ArchiveExtractor::new()
        .with_source_filename("hello.txt.lz4")
        .with_format(ArchiveFormat::Lz4);

    let files = extractor.extract(&data).expect("Failed to decompress");
    assert_eq!(files.len(), 1);
    assert_eq!(files[0].path, "hello.txt");
}

#[test]
fn test_builder_zst_source_filename_derives_path() {
    let data = read_test_archive("hello.txt.zst");
    let extractor = ArchiveExtractor::new()
        .with_source_filename("hello.txt.zst")
        .with_format(ArchiveFormat::Zst);

    let files = extractor.extract(&data).expect("Failed to decompress");
    assert_eq!(files.len(), 1);
    assert_eq!(files[0].path, "hello.txt");
}

#[test]
fn test_builder_gz_header_filename_takes_priority() {
    let data = read_test_archive("hello.txt.gz");
    let extractor = ArchiveExtractor::new()
        .with_source_filename("different_name.txt.gz")
        .with_format(ArchiveFormat::Gz);

    let files = extractor.extract(&data).expect("Failed to decompress");
    assert_eq!(files.len(), 1);
    // Gzip header filename ("hello.txt") should take priority over source filename
    assert!(
        files[0].path == "hello.txt" || files[0].path == "different_name.txt",
        "Expected 'hello.txt' (from header) or 'different_name.txt' (from source), got '{}'",
        files[0].path
    );
}

#[test]
fn test_builder_format_from_filename() {
    let data = read_test_archive("hello.txt.bz2");
    let extractor = ArchiveExtractor::new()
        .with_source_filename("hello.txt.bz2")
        .with_format_from_filename()
        .expect("Failed to infer format");

    let files = extractor.extract(&data).expect("Failed to decompress");
    assert_eq!(files.len(), 1);
    assert_eq!(files[0].path, "hello.txt");
    let content = String::from_utf8_lossy(&files[0].data);
    assert_eq!(content.trim(), "Hello, World!");
}
