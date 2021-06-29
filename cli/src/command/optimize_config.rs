use crate::{
    command::{ExonumCommand, StandardResult},
    config::NodeConfig,
    io::{load_config_file, save_config_file},
};
use anyhow::{bail, Error};
use serde_derive::{Deserialize, Serialize};
use std::{fs, path::PathBuf};
use structopt::StructOpt;

// 1 MiB = 2^20 bytes
pub const MEBIBYTE: u64 = 1<<20;

/// Maximum number of files that RocksDb may keep open.
pub const MAX_OPEN_FILES: i32 = 256;

/// Maximum size of RocksDb's WAL journal in bytes.
pub const MAX_TOTAL_WAL_SIZE: u64 = 1 * MEBIBYTE;

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
}

impl ExonumCommand for OptimizeConfig {
    fn execute(self) -> Result<StandardResult, Error> {
        // tune the settings from the previous configuration step
        let mut node_config: NodeConfig = load_config_file(&self.node_config_file)?;
        node_config.private_config.database.max_open_files =
            self.max_open_files.or(Some(MAX_OPEN_FILES));
        node_config.private_config.database.max_total_wal_size =
            self.max_total_wal_size.or(Some(MAX_TOTAL_WAL_SIZE));

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
