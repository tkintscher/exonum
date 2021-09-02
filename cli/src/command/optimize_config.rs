use crate::{
    command::{ExonumCommand, StandardResult},
    config::NodeConfig,
    io::{load_config_file, save_config_file},
};
use anyhow::{anyhow, bail, Error};
use exonum::merkledb::LogVerbosity;
use serde_derive::{Deserialize, Serialize};
use std::{fs, path::PathBuf};
use structopt::StructOpt;

/// Maximum number of files that RocksDb may keep open.
pub const MAX_OPEN_FILES: i32 = 256;

/// Maximum size of RocksDb's WAL journal in bytes (1 MiB).
pub const MAX_TOTAL_WAL_SIZE: u64 = 1 * (1 << 20);

// Default log level for RocksDb.
pub const DEFAULT_LOG_LEVEL: LogVerbosity = LogVerbosity::Warn;

/// Maximum size of RocksDb's info LOG in bytes (10 MiB).
pub const MAX_LOG_FILE_SIZE: usize = 10 * (1 << 20);

/// How many info LOG files to keep.
pub const KEEP_LOG_FILE_NUM: usize = 10;

/// Options for optimizing RocksDb.
#[derive(StructOpt, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct OptimizeConfig {
    /// Path to node configuration file (node.toml).
    pub node_config_file: PathBuf,

    /// Where to store the modified node configuration.
    ///
    /// Default: overwrite the input file.
    #[structopt(long, short = "o")]
    pub output_file: Option<PathBuf>,

    /// Maximum number of files that RocksDb may keep open.
    ///
    /// Defaults to 256.
    #[structopt(long)]
    pub max_open_files: Option<i32>,

    /// Maximum size of RocksDb's WAL journal in bytes.
    ///
    /// Defaults to 1 MiB.
    #[structopt(long)]
    pub max_total_wal_size: Option<u64>,

    /// Log level.
    ///
    /// Defaults to `Warn`.
    #[structopt(long, parse(try_from_str = parse_log_level))]
    pub log_level: Option<LogVerbosity>,

    /// Maximum size of log files.
    ///
    /// Defaults to 10 MiB.
    #[structopt(long)]
    pub max_log_file_size: Option<usize>,

    /// Maximum number of log files to keep.
    ///
    /// Defaults to 10.
    #[structopt(long)]
    pub keep_log_file_num: Option<usize>,

    /// Recycle existing log files.
    ///
    /// Defaults to true.
    #[structopt(long)]
    pub recycle_log_files: Option<bool>,
}

fn parse_log_level(src: &str) -> Result<LogVerbosity, Error> {
    match src.to_lowercase().as_ref() {
        "debug" => Ok(LogVerbosity::Debug),
        "info" => Ok(LogVerbosity::Info),
        "warn" => Ok(LogVerbosity::Warn),
        "error" => Ok(LogVerbosity::Error),
        "fatal" => Ok(LogVerbosity::Fatal),
        "header" => Ok(LogVerbosity::Header),
        _ => Err(anyhow!("Unknown log level: {}", src)),
    }
}

impl ExonumCommand for OptimizeConfig {
    fn execute(self) -> Result<StandardResult, Error> {
        // tune the settings from the previous configuration step
        let mut node_config: NodeConfig = load_config_file(&self.node_config_file)?;
        node_config.private_config.database.max_open_files =
            self.max_open_files.or(Some(MAX_OPEN_FILES));
        node_config.private_config.database.max_total_wal_size =
            self.max_total_wal_size.or(Some(MAX_TOTAL_WAL_SIZE));
        node_config.private_config.database.log_verbosity =
            self.log_level.or(Some(DEFAULT_LOG_LEVEL));
        node_config.private_config.database.max_log_file_size =
            self.max_log_file_size.or(Some(MAX_LOG_FILE_SIZE));
        node_config.private_config.database.keep_log_file_num =
            self.keep_log_file_num.or(Some(KEEP_LOG_FILE_NUM));
        node_config.private_config.database.recycle_log_file_num =
            self.recycle_log_files.map(|value| {
                if value {
                    1
                } else {
                    0
                }
            } as usize);

        // Since this may overwrite the input file, we aim for consistency
        // by first writing to a temporary file, then moving atomically.
        let out_file = self.output_file.unwrap_or(self.node_config_file.clone());
        let tmp_file = out_file.with_extension(".tmp");
        if tmp_file.exists() {
            bail!(
                "Failed to write to temporary output file. File exists: {:?}",
                tmp_file
            )
        }
        save_config_file(&node_config, &tmp_file)?;
        fs::rename(tmp_file, &out_file)?;

        Ok(StandardResult::OptimizeConfig {
            node_config_path: out_file,
        })
    }
}
