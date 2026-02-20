//! Archive format identification.
//!
//! This module defines the supported archive and compression formats.

use mime_type::MimeType;

use crate::ArchiveError;

/// Supported archive and compression formats.
///
/// This enum represents all archive and compression formats that can be extracted
/// by this crate. It includes multi-file archives (ZIP, TAR, 7-Zip) and single-file
/// compression formats (gzip, bzip2, etc.).
///
/// # Examples
///
/// ```no_run
/// use archive::{ArchiveExtractor, ArchiveFormat};
///
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let extractor = ArchiveExtractor::new();
///
/// // Extract a ZIP archive
/// # let zip_data = vec![0u8; 100];
/// let files = extractor.extract_with_format(&zip_data, ArchiveFormat::Zip)?;
///
/// // Extract a gzip-compressed TAR archive
/// # let targz_data = vec![0u8; 100];
/// let files = extractor.extract_with_format(&targz_data, ArchiveFormat::TarGz)?;
///
/// // Decompress a single gzip file
/// # let gz_data = vec![0u8; 100];
/// let files = extractor.extract_with_format(&gz_data, ArchiveFormat::Gz)?;
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArchiveFormat {
    /// ZIP archive format (`.zip`).
    ///
    /// ZIP is a widely-used archive format that supports multiple compression
    /// methods and can store multiple files with directory structure.
    ///
    /// Supports various compression levels including store (no compression),
    /// deflate, and others.
    Zip,

    /// Plain TAR archive (`.tar`).
    ///
    /// TAR (Tape Archive) is a file format for collecting multiple files into
    /// a single archive file. This variant is uncompressed.
    Tar,

    /// Unix ar archive (`.ar`).
    ///
    /// ar (archive) is a file format for collecting multiple files into
    /// a single archive file. The file format is used commonly on unix-like
    /// systems, but the file format itself has never been standardized and
    /// there are multiple variants of the format.
    Ar,

    /// Debian package (`.deb`).
    ///
    /// Debian packages are a file format used by the Debian package management
    /// system. They are based on the ar archive format and contain metadata
    /// about the package, as well as the actual files to be installed.
    Deb,

    /// TAR archive with gzip compression (`.tar.gz`, `.tgz`).
    ///
    /// Combines TAR archiving with gzip compression. This is one of the most
    /// common formats on Unix-like systems.
    TarGz,

    /// TAR archive with bzip2 compression (`.tar.bz2`, `.tbz2`).
    ///
    /// Combines TAR archiving with bzip2 compression, which typically provides
    /// better compression ratios than gzip but is slower.
    TarBz2,

    /// TAR archive with XZ/LZMA compression (`.tar.xz`, `.txz`).
    ///
    /// Combines TAR archiving with XZ compression (based on LZMA), which provides
    /// excellent compression ratios but requires more memory and CPU time.
    TarXz,

    /// TAR archive with Zstandard compression (`.tar.zst`).
    ///
    /// Combines TAR archiving with Zstandard compression, which offers a good
    /// balance between compression ratio and speed.
    TarZst,

    /// TAR archive with LZ4 compression (`.tar.lz4`).
    ///
    /// Combines TAR archiving with LZ4 compression, which prioritizes speed
    /// over compression ratio. Useful for fast decompression.
    TarLz4,

    /// Single file compressed with gzip (`.gz`).
    ///
    /// A single file compressed using the gzip algorithm. If the gzip header
    /// contains the original filename, it will be preserved during extraction;
    /// otherwise, the file will be named "data".
    Gz,

    /// Single file compressed with bzip2 (`.bz2`).
    ///
    /// A single file compressed using the bzip2 algorithm. The extracted file
    /// will be named "data" as bzip2 doesn't store original filenames.
    Bz2,

    /// Single file compressed with XZ/LZMA (`.xz`).
    ///
    /// A single file compressed using the XZ algorithm (based on LZMA).
    /// The extracted file will be named "data" as XZ doesn't store original filenames.
    Xz,

    /// Single file compressed with LZ4 (`.lz4`).
    ///
    /// A single file compressed using the LZ4 algorithm. The extracted file
    /// will be named "data" as LZ4 doesn't store original filenames.
    Lz4,

    /// Single file compressed with Zstandard (`.zst`).
    ///
    /// A single file compressed using the Zstandard algorithm. The extracted file
    /// will be named "data" as Zstandard doesn't store original filenames by default.
    Zst,

    /// 7-Zip archive format (`.7z`).
    ///
    /// 7-Zip is a high-compression archive format that supports multiple
    /// compression algorithms and can achieve excellent compression ratios.
    SevenZ,
}

impl ArchiveFormat {
    /// Determines the archive format from a filename's extension.
    ///
    /// Performs case-insensitive matching. Double extensions (e.g. `.tar.gz`)
    /// are checked before single extensions.
    ///
    /// # Errors
    ///
    /// Returns [`ArchiveError::UnknownFormat`] if the extension is not recognized.
    ///
    /// # Examples
    ///
    /// ```
    /// use archive::ArchiveFormat;
    ///
    /// assert_eq!(ArchiveFormat::from_filename("archive.tar.gz").unwrap(), ArchiveFormat::TarGz);
    /// assert_eq!(ArchiveFormat::from_filename("FILE.ZIP").unwrap(), ArchiveFormat::Zip);
    /// assert!(ArchiveFormat::from_filename("readme.txt").is_err());
    /// ```
    pub fn from_filename(filename: &str) -> Result<Self, ArchiveError> {
        let lower = filename.to_lowercase();

        // Check double extensions first
        if lower.ends_with(".tar.gz") {
            return Ok(Self::TarGz);
        }
        if lower.ends_with(".tar.bz2") {
            return Ok(Self::TarBz2);
        }
        if lower.ends_with(".tar.xz") {
            return Ok(Self::TarXz);
        }
        if lower.ends_with(".tar.zst") {
            return Ok(Self::TarZst);
        }
        if lower.ends_with(".tar.lz4") {
            return Ok(Self::TarLz4);
        }

        // Check single extensions
        let ext = lower.rsplit('.').next().unwrap_or("");
        match ext {
            "zip" => Ok(Self::Zip),
            "tar" => Ok(Self::Tar),
            "ar" => Ok(Self::Ar),
            "deb" => Ok(Self::Deb),
            "tgz" => Ok(Self::TarGz),
            "tbz2" => Ok(Self::TarBz2),
            "txz" => Ok(Self::TarXz),
            "gz" => Ok(Self::Gz),
            "bz2" => Ok(Self::Bz2),
            "xz" => Ok(Self::Xz),
            "lz4" => Ok(Self::Lz4),
            "zst" => Ok(Self::Zst),
            "7z" => Ok(Self::SevenZ),
            _ => Err(ArchiveError::UnknownFormat),
        }
    }

    /// Returns the human-readable name of the archive format.
    ///
    /// This method returns a string representation of the format, suitable
    /// for display purposes.
    ///
    /// # Examples
    ///
    /// ```
    /// use archive::ArchiveFormat;
    ///
    /// assert_eq!(ArchiveFormat::Zip.name(), "ZIP");
    /// assert_eq!(ArchiveFormat::TarGz.name(), "TAR.GZ");
    /// assert_eq!(ArchiveFormat::SevenZ.name(), "7Z");
    /// ```
    pub fn name(&self) -> &'static str {
        match self {
            Self::Zip => "ZIP",
            Self::Tar => "TAR",
            Self::Ar => "AR",
            Self::Deb => "DEB",
            Self::TarGz => "TAR.GZ",
            Self::TarBz2 => "TAR.BZ2",
            Self::TarXz => "TAR.XZ",
            Self::TarZst => "TAR.ZST",
            Self::TarLz4 => "TAR.LZ4",
            Self::Gz => "GZIP",
            Self::Bz2 => "BZIP2",
            Self::Xz => "XZ",
            Self::Lz4 => "LZ4",
            Self::Zst => "ZSTD",
            Self::SevenZ => "7Z",
        }
    }

    /// Determines the archive format from a MIME type string.
    ///
    /// Performs case-insensitive matching against known archive MIME types.
    ///
    /// Note that MIME types alone cannot distinguish `TarGz` from `Gz`,
    /// `TarBz2` from `Bz2`, etc. — this method returns the base compression
    /// format. Combine with [`from_filename`](Self::from_filename) when you
    /// need tar+compression detection.
    ///
    /// # Errors
    ///
    /// Returns [`ArchiveError::UnsupportedFormat`] if the MIME type is not recognized.
    ///
    /// # Examples
    ///
    /// ```
    /// use archive::ArchiveFormat;
    ///
    /// assert_eq!(ArchiveFormat::from_mime_str("application/zip").unwrap(), ArchiveFormat::Zip);
    /// assert_eq!(ArchiveFormat::from_mime_str("application/gzip").unwrap(), ArchiveFormat::Gz);
    /// assert_eq!(ArchiveFormat::from_mime_str("APPLICATION/ZIP").unwrap(), ArchiveFormat::Zip);
    /// assert!(ArchiveFormat::from_mime_str("text/plain").is_err());
    /// ```
    pub fn from_mime_str(mime: &str) -> Result<Self, ArchiveError> {
        match mime.to_ascii_lowercase().as_str() {
            "application/zip" => Ok(Self::Zip),
            "application/x-tar" => Ok(Self::Tar),
            "application/x-ar" => Ok(Self::Ar),
            "application/vnd.debian.binary-package" => Ok(Self::Deb),
            "application/gzip" | "application/x-gzip" => Ok(Self::Gz),
            "application/x-bzip2" | "application/x-bzip" => Ok(Self::Bz2),
            "application/x-xz" => Ok(Self::Xz),
            "application/x-lz4" => Ok(Self::Lz4),
            "application/zstd" | "application/x-zstd" => Ok(Self::Zst),
            "application/x-7z-compressed" => Ok(Self::SevenZ),
            other => Err(ArchiveError::UnsupportedFormat(other.to_string())),
        }
    }

    /// Detects the archive format from file content using magic byte signatures.
    ///
    /// This method is available when either the `detect-libmagic` or `detect-infer`
    /// feature is enabled. If both are enabled, `detect-libmagic` takes priority.
    ///
    /// # Errors
    ///
    /// Returns [`ArchiveError::UnknownFormat`] if the format cannot be detected.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use archive::ArchiveFormat;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let data = std::fs::read("archive.zip")?;
    /// let format = ArchiveFormat::from_bytes(&data)?;
    /// assert_eq!(format, ArchiveFormat::Zip);
    /// # Ok(())
    /// # }
    /// ```
    #[cfg(feature = "detect-libmagic")]
    pub fn from_bytes(data: &[u8]) -> Result<Self, ArchiveError> {
        let cookie = magic::Cookie::open(magic::CookieFlags::MIME_TYPE)
            .map_err(|e| ArchiveError::InvalidArchive(format!("libmagic error: {e}")))?;
        cookie
            .load::<&str>(&[])
            .map_err(|e| ArchiveError::InvalidArchive(format!("libmagic load error: {e}")))?;
        let mime = cookie
            .buffer(data)
            .map_err(|e| ArchiveError::InvalidArchive(format!("libmagic buffer error: {e}")))?;
        Self::from_mime_str(&mime).map_err(|_| ArchiveError::UnknownFormat)
    }

    /// Detects the archive format from file content using magic byte signatures.
    ///
    /// This method is available when either the `detect-libmagic` or `detect-infer`
    /// feature is enabled. If both are enabled, `detect-libmagic` takes priority.
    ///
    /// # Errors
    ///
    /// Returns [`ArchiveError::UnknownFormat`] if the format cannot be detected.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use archive::ArchiveFormat;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let data = std::fs::read("archive.zip")?;
    /// let format = ArchiveFormat::from_bytes(&data)?;
    /// assert_eq!(format, ArchiveFormat::Zip);
    /// # Ok(())
    /// # }
    /// ```
    #[cfg(all(feature = "detect-infer", not(feature = "detect-libmagic")))]
    pub fn from_bytes(data: &[u8]) -> Result<Self, ArchiveError> {
        let kind = infer::get(data).ok_or(ArchiveError::UnknownFormat)?;
        Self::from_mime_str(kind.mime_type()).map_err(|_| ArchiveError::UnknownFormat)
    }

    /// Checks if a given MIME type corresponds to a supported archive format.
    ///
    /// This method attempts to convert the provided MIME type into an
    /// `ArchiveFormat`. If the conversion is successful, it indicates that
    /// the MIME type is supported.
    ///
    /// # Examples
    /// ```
    /// use archive::{ArchiveFormat};
    /// use mime_type::{MimeType, MimeFormat, Application};
    ///
    /// let mime_zip = MimeType::Archive(mime_type::Archive::Zip);
    /// let mime_gz = MimeType::Archive(mime_type::Archive::Gz);
    /// let mime_unknown = MimeType::from_mime("application/octet-stream").unwrap();
    ///
    /// assert!(ArchiveFormat::is_supported_mime(&mime_zip));
    /// assert!(ArchiveFormat::is_supported_mime(&mime_gz));
    /// assert!(!ArchiveFormat::is_supported_mime(&mime_unknown));
    /// ```
    pub fn is_supported_mime(mime: &MimeType) -> bool {
        ArchiveFormat::try_from(mime).is_ok()
    }
}

impl TryFrom<&MimeType> for ArchiveFormat {
    type Error = ArchiveError;

    fn try_from(mime: &MimeType) -> Result<Self, Self::Error> {
        match mime {
            MimeType::Archive(mime_type::Archive::Zip) => Ok(Self::Zip),
            MimeType::Archive(mime_type::Archive::Tar) => Ok(Self::Tar),
            MimeType::Archive(mime_type::Archive::Ar) => Ok(Self::Ar),
            MimeType::Archive(mime_type::Archive::Deb) => Ok(Self::Deb),
            MimeType::Archive(mime_type::Archive::Gz) => Ok(Self::Gz),
            MimeType::Archive(mime_type::Archive::Bz2) => Ok(Self::Bz2),
            MimeType::Archive(mime_type::Archive::Xz) => Ok(Self::Xz),
            MimeType::Archive(mime_type::Archive::Lz4) => Ok(Self::Lz4),
            MimeType::Archive(mime_type::Archive::Zst) => Ok(Self::Zst),
            MimeType::Archive(mime_type::Archive::SevenZ) => Ok(Self::SevenZ),
            _ => Err(ArchiveError::UnsupportedFormat(mime.to_string())),
        }
    }
}

impl TryFrom<MimeType> for ArchiveFormat {
    type Error = ArchiveError;

    fn try_from(mime: MimeType) -> Result<Self, Self::Error> {
        ArchiveFormat::try_from(&mime)
    }
}

impl From<&ArchiveFormat> for MimeType {
    fn from(format: &ArchiveFormat) -> Self {
        match format {
            ArchiveFormat::Zip => MimeType::Archive(mime_type::Archive::Zip),
            ArchiveFormat::Tar => MimeType::Archive(mime_type::Archive::Tar),
            ArchiveFormat::Ar => MimeType::Archive(mime_type::Archive::Ar),
            ArchiveFormat::Deb => MimeType::Archive(mime_type::Archive::Deb),
            ArchiveFormat::Gz => MimeType::Archive(mime_type::Archive::Gz),
            ArchiveFormat::Bz2 => MimeType::Archive(mime_type::Archive::Bz2),
            ArchiveFormat::Xz => MimeType::Archive(mime_type::Archive::Xz),
            ArchiveFormat::Lz4 => MimeType::Archive(mime_type::Archive::Lz4),
            ArchiveFormat::Zst => MimeType::Archive(mime_type::Archive::Zst),
            ArchiveFormat::SevenZ => MimeType::Archive(mime_type::Archive::SevenZ),
            ArchiveFormat::TarGz => MimeType::Archive(mime_type::Archive::Gz),
            ArchiveFormat::TarBz2 => MimeType::Archive(mime_type::Archive::Bz2),
            ArchiveFormat::TarXz => MimeType::Archive(mime_type::Archive::Xz),
            ArchiveFormat::TarZst => MimeType::Archive(mime_type::Archive::Zst),
            ArchiveFormat::TarLz4 => MimeType::Archive(mime_type::Archive::Lz4),
        }
    }
}

impl From<ArchiveFormat> for MimeType {
    fn from(format: ArchiveFormat) -> Self {
        MimeType::from(&format)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_filename_all_extensions() {
        assert_eq!(
            ArchiveFormat::from_filename("a.zip").unwrap(),
            ArchiveFormat::Zip
        );
        assert_eq!(
            ArchiveFormat::from_filename("a.tar").unwrap(),
            ArchiveFormat::Tar
        );
        assert_eq!(
            ArchiveFormat::from_filename("a.ar").unwrap(),
            ArchiveFormat::Ar
        );
        assert_eq!(
            ArchiveFormat::from_filename("a.deb").unwrap(),
            ArchiveFormat::Deb
        );
        assert_eq!(
            ArchiveFormat::from_filename("a.tar.gz").unwrap(),
            ArchiveFormat::TarGz
        );
        assert_eq!(
            ArchiveFormat::from_filename("a.tgz").unwrap(),
            ArchiveFormat::TarGz
        );
        assert_eq!(
            ArchiveFormat::from_filename("a.tar.bz2").unwrap(),
            ArchiveFormat::TarBz2
        );
        assert_eq!(
            ArchiveFormat::from_filename("a.tbz2").unwrap(),
            ArchiveFormat::TarBz2
        );
        assert_eq!(
            ArchiveFormat::from_filename("a.tar.xz").unwrap(),
            ArchiveFormat::TarXz
        );
        assert_eq!(
            ArchiveFormat::from_filename("a.txz").unwrap(),
            ArchiveFormat::TarXz
        );
        assert_eq!(
            ArchiveFormat::from_filename("a.tar.zst").unwrap(),
            ArchiveFormat::TarZst
        );
        assert_eq!(
            ArchiveFormat::from_filename("a.tar.lz4").unwrap(),
            ArchiveFormat::TarLz4
        );
        assert_eq!(
            ArchiveFormat::from_filename("a.gz").unwrap(),
            ArchiveFormat::Gz
        );
        assert_eq!(
            ArchiveFormat::from_filename("a.bz2").unwrap(),
            ArchiveFormat::Bz2
        );
        assert_eq!(
            ArchiveFormat::from_filename("a.xz").unwrap(),
            ArchiveFormat::Xz
        );
        assert_eq!(
            ArchiveFormat::from_filename("a.lz4").unwrap(),
            ArchiveFormat::Lz4
        );
        assert_eq!(
            ArchiveFormat::from_filename("a.zst").unwrap(),
            ArchiveFormat::Zst
        );
        assert_eq!(
            ArchiveFormat::from_filename("a.7z").unwrap(),
            ArchiveFormat::SevenZ
        );
    }

    #[test]
    fn test_from_filename_case_insensitive() {
        assert_eq!(
            ArchiveFormat::from_filename("FILE.ZIP").unwrap(),
            ArchiveFormat::Zip
        );
        assert_eq!(
            ArchiveFormat::from_filename("Archive.Tar.Gz").unwrap(),
            ArchiveFormat::TarGz
        );
        assert_eq!(
            ArchiveFormat::from_filename("DATA.BZ2").unwrap(),
            ArchiveFormat::Bz2
        );
        assert_eq!(
            ArchiveFormat::from_filename("backup.TAR.XZ").unwrap(),
            ArchiveFormat::TarXz
        );
    }

    #[test]
    fn test_from_filename_double_extensions() {
        // Double extensions should match before single
        assert_eq!(
            ArchiveFormat::from_filename("foo.tar.gz").unwrap(),
            ArchiveFormat::TarGz
        );
        assert_eq!(
            ArchiveFormat::from_filename("foo.tar.bz2").unwrap(),
            ArchiveFormat::TarBz2
        );
        assert_eq!(
            ArchiveFormat::from_filename("foo.tar.xz").unwrap(),
            ArchiveFormat::TarXz
        );
        assert_eq!(
            ArchiveFormat::from_filename("foo.tar.zst").unwrap(),
            ArchiveFormat::TarZst
        );
        assert_eq!(
            ArchiveFormat::from_filename("foo.tar.lz4").unwrap(),
            ArchiveFormat::TarLz4
        );
    }

    #[test]
    fn test_from_filename_unknown_extension() {
        assert!(ArchiveFormat::from_filename("readme.txt").is_err());
        assert!(ArchiveFormat::from_filename("photo.png").is_err());
        assert!(ArchiveFormat::from_filename("noextension").is_err());
    }

    #[test]
    fn test_from_mime_str_all_supported() {
        assert_eq!(
            ArchiveFormat::from_mime_str("application/zip").unwrap(),
            ArchiveFormat::Zip
        );
        assert_eq!(
            ArchiveFormat::from_mime_str("application/x-tar").unwrap(),
            ArchiveFormat::Tar
        );
        assert_eq!(
            ArchiveFormat::from_mime_str("application/x-ar").unwrap(),
            ArchiveFormat::Ar
        );
        assert_eq!(
            ArchiveFormat::from_mime_str("application/vnd.debian.binary-package").unwrap(),
            ArchiveFormat::Deb
        );
        assert_eq!(
            ArchiveFormat::from_mime_str("application/gzip").unwrap(),
            ArchiveFormat::Gz
        );
        assert_eq!(
            ArchiveFormat::from_mime_str("application/x-bzip2").unwrap(),
            ArchiveFormat::Bz2
        );
        assert_eq!(
            ArchiveFormat::from_mime_str("application/x-xz").unwrap(),
            ArchiveFormat::Xz
        );
        assert_eq!(
            ArchiveFormat::from_mime_str("application/x-lz4").unwrap(),
            ArchiveFormat::Lz4
        );
        assert_eq!(
            ArchiveFormat::from_mime_str("application/zstd").unwrap(),
            ArchiveFormat::Zst
        );
        assert_eq!(
            ArchiveFormat::from_mime_str("application/x-7z-compressed").unwrap(),
            ArchiveFormat::SevenZ
        );
    }

    #[test]
    fn test_from_mime_str_case_insensitive() {
        assert_eq!(
            ArchiveFormat::from_mime_str("APPLICATION/ZIP").unwrap(),
            ArchiveFormat::Zip
        );
        assert_eq!(
            ArchiveFormat::from_mime_str("Application/Gzip").unwrap(),
            ArchiveFormat::Gz
        );
        assert_eq!(
            ArchiveFormat::from_mime_str("APPLICATION/X-TAR").unwrap(),
            ArchiveFormat::Tar
        );
    }

    #[test]
    fn test_from_mime_str_aliases() {
        assert_eq!(
            ArchiveFormat::from_mime_str("application/x-gzip").unwrap(),
            ArchiveFormat::Gz
        );
        assert_eq!(
            ArchiveFormat::from_mime_str("application/x-bzip").unwrap(),
            ArchiveFormat::Bz2
        );
        assert_eq!(
            ArchiveFormat::from_mime_str("application/x-zstd").unwrap(),
            ArchiveFormat::Zst
        );
    }

    #[test]
    fn test_from_mime_str_unknown() {
        assert!(ArchiveFormat::from_mime_str("text/plain").is_err());
        assert!(ArchiveFormat::from_mime_str("application/octet-stream").is_err());
        assert!(ArchiveFormat::from_mime_str("image/png").is_err());
    }

    #[cfg(feature = "detect-infer")]
    #[test]
    fn test_from_bytes_zip() {
        // Create a minimal ZIP in memory
        let buf = Vec::new();
        let cursor = std::io::Cursor::new(buf);
        let mut writer = zip::ZipWriter::new(cursor);
        let options = zip::write::SimpleFileOptions::default();
        writer.start_file("test.txt", options).unwrap();
        std::io::Write::write_all(&mut writer, b"hello").unwrap();
        let cursor = writer.finish().unwrap();
        let data = cursor.into_inner();

        assert_eq!(
            ArchiveFormat::from_bytes(&data).unwrap(),
            ArchiveFormat::Zip
        );
    }

    #[cfg(feature = "detect-infer")]
    #[test]
    fn test_from_bytes_gz() {
        use flate2::Compression;
        use flate2::write::GzEncoder;
        let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
        std::io::Write::write_all(&mut encoder, b"hello").unwrap();
        let data = encoder.finish().unwrap();

        assert_eq!(ArchiveFormat::from_bytes(&data).unwrap(), ArchiveFormat::Gz);
    }

    #[cfg(feature = "detect-infer")]
    #[test]
    fn test_from_bytes_bz2() {
        let mut encoder = bzip2::write::BzEncoder::new(Vec::new(), bzip2::Compression::default());
        std::io::Write::write_all(&mut encoder, b"hello").unwrap();
        let data = encoder.finish().unwrap();

        assert_eq!(
            ArchiveFormat::from_bytes(&data).unwrap(),
            ArchiveFormat::Bz2
        );
    }

    #[cfg(feature = "detect-infer")]
    #[test]
    fn test_from_bytes_xz() {
        let mut output = Vec::new();
        lzma_rs::xz_compress(&mut std::io::Cursor::new(b"hello"), &mut output).unwrap();

        assert_eq!(
            ArchiveFormat::from_bytes(&output).unwrap(),
            ArchiveFormat::Xz
        );
    }

    #[cfg(feature = "detect-infer")]
    #[test]
    fn test_from_bytes_zst() {
        let data = zstd::encode_all(std::io::Cursor::new(b"hello"), 3).unwrap();

        assert_eq!(
            ArchiveFormat::from_bytes(&data).unwrap(),
            ArchiveFormat::Zst
        );
    }

    #[cfg(feature = "detect-infer")]
    #[test]
    fn test_from_bytes_unknown() {
        assert!(ArchiveFormat::from_bytes(b"just some random text").is_err());
    }
}
