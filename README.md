# archive

[![Crates.io](https://img.shields.io/crates/v/archive.svg)](https://crates.io/crates/archive)
[![Documentation](https://docs.rs/archive/badge.svg)](https://docs.rs/archive)
[![CI](https://github.com/secana/archive/workflows/Archive%20CI/badge.svg)](https://github.com/secana/archive/actions)

A unified, pure-Rust interface for extracting common archive formats in-memory.

    This crate is currently in development and should not be used in production.
    The API may change in future releases.

## Features

- **Unified API**: Single interface for all archive formats
- **In-memory extraction**: No disk I/O required
- **Safety limits**: Protection against zip bombs and resource exhaustion
- **Pure Rust**: Minimal C dependencies (only bzip2)
- **Cross-platform**: Works on Linux, macOS, Windows (x86_64, ARM64)

### Supported Formats

| Format | Extensions | Description |
|--------|------------|-------------|
| **ZIP** | `.zip` | ZIP archives with various compression levels |
| **TAR** | `.tar` | Uncompressed TAR archives |
| **AR** | `.ar` | Uncompressed AR archives |
| **DEB** | `.deb` | Debian packages (which are also AR archives) |
| **TAR.GZ** | `.tar.gz`, `.tgz` | TAR with gzip compression |
| **TAR.BZ2** | `.tar.bz2`, `.tbz2` | TAR with bzip2 compression |
| **TAR.XZ** | `.tar.xz`, `.txz` | TAR with xz/LZMA compression |
| **TAR.ZST** | `.tar.zst` | TAR with Zstandard compression |
| **TAR.LZ4** | `.tar.lz4` | TAR with LZ4 compression |
| **7-Zip** | `.7z` | 7-Zip archives |
| **Single-file** | `.gz`, `.bz2`, `.xz`, `.lz4`, `.zst` | Individual compressed files |

## Usage

### Builder API (recommended)

Configure the extractor with a format and optional source filename, then call `extract()`:

```rust
use archive::ArchiveExtractor;
use std::fs;

let path = "backup.tar.gz";
let data = fs::read(path)?;

let extractor = ArchiveExtractor::new()
    .with_source_filename(path)
    .with_format_from_filename()?;

let files = extractor.extract(&data)?;

for file in &files {
    if file.is_directory {
        println!("[dir]  {}", file.path);
    } else {
        println!("[file] {} ({} bytes)", file.path, file.data.len());
    }
}
```

You can also set the format explicitly:

```rust
use archive::{ArchiveExtractor, ArchiveFormat};

let extractor = ArchiveExtractor::new()
    .with_format(ArchiveFormat::Zip);

let files = extractor.extract(&data)?;
```

### Direct format extraction

If you don't need the builder, pass the format directly with `extract_with_format()`:

```rust
use archive::{ArchiveExtractor, ArchiveFormat};

let extractor = ArchiveExtractor::new();
let files = extractor.extract_with_format(&data, ArchiveFormat::SevenZ)?;
```

### Single-file decompression with derived output paths

When `source_filename` is set and the format is a single-file compressor (Gz, Bz2, Xz, Lz4, Zst), the output path is derived by stripping the compression extension:

```rust
use archive::{ArchiveExtractor, ArchiveFormat};

let extractor = ArchiveExtractor::new()
    .with_source_filename("report.csv.bz2")
    .with_format(ArchiveFormat::Bz2);

let files = extractor.extract(&data)?;
assert_eq!(files[0].path, "report.csv"); // stripped ".bz2"
```

Without `source_filename`, the path defaults to `"data"` (except for gzip, which reads the original filename from the header first).

### Size limits

Protect against zip bombs and resource exhaustion with configurable limits:

```rust
use archive::{ArchiveExtractor, ArchiveFormat};

let extractor = ArchiveExtractor::new()
    .with_max_file_size(10 * 1024 * 1024)    // 10 MB per file
    .with_max_total_size(100 * 1024 * 1024)   // 100 MB total
    .with_format(ArchiveFormat::Zip);

let files = extractor.extract(&data)?;
```

Default limits are 100 MB per file and 1 GB total.

### Inspecting archive contents

```rust
use archive::ArchiveExtractor;

let extractor = ArchiveExtractor::new()
    .with_source_filename("project.tar.xz")
    .with_format_from_filename()?;

let files = extractor.extract(&data)?;

let dirs: Vec<_> = files.iter().filter(|f| f.is_directory).collect();
let regular: Vec<_> = files.iter().filter(|f| !f.is_directory).collect();
let total_bytes: usize = regular.iter().map(|f| f.data.len()).sum();

println!("{} directories, {} files, {} bytes", dirs.len(), regular.len(), total_bytes);
```

## Migration from v0.3

### Quick migration: find and replace

The only breaking change is that `extract(data, format)` was renamed to `extract_with_format(data, format)`. A mechanical find-and-replace is sufficient:

```bash
# Preview changes
grep -rn '\.extract(' src/ tests/ --include='*.rs'

# Apply rename
find src/ tests/ -name '*.rs' -exec \
  sed -i 's/\.extract(\([^)]*\), \(ArchiveFormat[^)]*\))/.extract_with_format(\1, \2)/g' {} +
```

Or in your editor, find `.extract(` calls that take two arguments (data and an `ArchiveFormat`) and replace with `.extract_with_format(`.

```rust
// Before (v0.3)
let files = extractor.extract(&data, ArchiveFormat::Zip)?;

// After (v0.4) — identical behavior, just renamed
let files = extractor.extract_with_format(&data, ArchiveFormat::Zip)?;
```

### Adopting the new builder API

Once your code compiles with `extract_with_format`, you can optionally refactor call sites to use the builder pattern. This is most useful when:

- You already know the filename and want automatic format detection
- You're decompressing single files and want meaningful output paths
- You want to configure the extractor once and reuse it

```rust
// Old style — still works
let files = extractor.extract_with_format(&data, ArchiveFormat::TarGz)?;

// Builder with explicit format
let extractor = ArchiveExtractor::new()
    .with_format(ArchiveFormat::TarGz);
let files = extractor.extract(&data)?;

// Builder with format inferred from filename
let extractor = ArchiveExtractor::new()
    .with_source_filename("backup.tar.gz")
    .with_format_from_filename()?;
let files = extractor.extract(&data)?;
```

### API changes summary

| v0.3 | v0.4 | Notes |
|------|------|-------|
| `extract(&data, format)` | `extract_with_format(&data, format)` | Renamed only |
| — | `extract(&data)` | Uses builder-configured format |
| — | `with_format(format)` | Set format on builder |
| — | `with_source_filename(name)` | Set source filename |
| — | `with_format_from_filename()` | Infer format from filename |
| — | `ArchiveFormat::from_filename(name)` | Extension-to-format mapping |

## Generate test archives

To generate the test archives used in this repository, you can use the provided Nix shell. First, ensure you have Nix installed on your system. Then, run the following commands:

```sh
nix run .#generateTestArchives

cargo test
```
