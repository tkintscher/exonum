// Copyright 2020 The Exonum Team
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Abstract settings for databases.

use rocksdb::{DBCompressionType, LogLevel};
use serde_derive::{Deserialize, Serialize};

/// Options for the database.
///
/// These parameters apply to the underlying database of Exonum, currently `RocksDB`.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[non_exhaustive]
pub struct DbOptions {
    /// Number of open files that can be used by the database.
    ///
    /// The underlying database opens multiple files during operation. If your system has a
    /// limit on the number of files which can be open simultaneously, you can
    /// adjust this option to match the limit. Note, that limiting the number
    /// of simultaneously open files might slow down the speed of database operation.
    ///
    /// Defaults to `None`, meaning that the number of open files is unlimited.
    pub max_open_files: Option<i32>,
    /// An option to indicate whether the system should create a database or not,
    /// if it's missing.
    ///
    /// This option applies to the cases when a node was
    /// switched off and is on again. If the database cannot be found at the
    /// indicated path and this option is switched on, a new database will be
    /// created at that path and blocks will be included therein.
    ///
    /// Defaults to `true`.
    pub create_if_missing: bool,
    /// An algorithm used for database compression.
    ///
    /// Defaults to `CompressionType::None`, meaning there is no compression.
    pub compression_type: CompressionType,
    /// Max total size of the WAL journal in bytes.
    ///
    /// Defaults to `None`, meaning that the size of WAL journal will be adjusted
    /// by the rocksdb.
    pub max_total_wal_size: Option<u64>,
    /// Verbosity of the LOG.
    ///
    /// Defaults to `Info`.
    pub log_verbosity: Option<LogVerbosity>,
    /// Maximal size of the info log file. If the file is larger than this, a new info log file
    /// will be created.
    ///
    /// Defaults to `0`, all logs will be written to the same file.
    pub max_log_file_size: Option<usize>,
    /// Maximum number of info log files to be kept.
    ///
    /// Default: 1000
    pub keep_log_file_num: Option<usize>,
    /// Recycle log files. If non-zero, previously written log files will be reused.
    ///
    /// Defaults to `0`, log files will not be reused.
    pub recycle_log_file_num: Option<usize>,
}

impl DbOptions {
    /// Creates a new `DbOptions` object.
    pub fn new(
        max_open_files: Option<i32>,
        create_if_missing: bool,
        compression_type: CompressionType,
        max_total_wal_size: Option<u64>,
        log_verbosity: Option<LogVerbosity>,
        max_log_file_size: Option<usize>,
        keep_log_file_num: Option<usize>,
        recycle_log_file_num: Option<usize>,
    ) -> Self {
        Self {
            max_open_files,
            create_if_missing,
            compression_type,
            max_total_wal_size,
            log_verbosity,
            max_log_file_size,
            keep_log_file_num,
            recycle_log_file_num,
        }
    }
}

/// Log levels.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
#[allow(missing_docs)]
pub enum LogVerbosity {
    Debug,
    Info,
    Warn,
    Error,
    Fatal,
    Header,
}

impl From<LogVerbosity> for LogLevel {
    fn from(level: LogVerbosity) -> Self {
        match level {
            LogVerbosity::Debug => Self::Debug,
            LogVerbosity::Info => Self::Info,
            LogVerbosity::Warn => Self::Warn,
            LogVerbosity::Error => Self::Error,
            LogVerbosity::Fatal => Self::Fatal,
            LogVerbosity::Header => Self::Header,
        }
    }
}

/// Algorithms of compression for the database.
///
/// Database contents are stored in a set of blocks, each of which holds a
/// sequence of key-value pairs. Each block may be compressed before
/// being stored in a file. The following enum describes which
/// compression algorithm (if any) is used to compress a block.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
#[allow(missing_docs)]
pub enum CompressionType {
    Bz2,
    Lz4,
    Lz4hc,
    Snappy,
    Zlib,
    Zstd,
    None,
}

impl From<CompressionType> for DBCompressionType {
    fn from(compression_type: CompressionType) -> Self {
        match compression_type {
            CompressionType::Bz2 => Self::Bz2,
            CompressionType::Lz4 => Self::Lz4,
            CompressionType::Lz4hc => Self::Lz4hc,
            CompressionType::Snappy => Self::Snappy,
            CompressionType::Zlib => Self::Zlib,
            CompressionType::Zstd => Self::Zstd,
            CompressionType::None => Self::None,
        }
    }
}

impl Default for DbOptions {
    fn default() -> Self {
        Self::new(
            None,
            true,
            CompressionType::None,
            None,
            None,
            None,
            None,
            None,
        )
    }
}
