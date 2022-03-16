// Copyright 2020 Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! Configurations used in the snapshotting context.

use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// The snapshot type options that are available when
/// creating a new snapshot.
#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub enum SnapshotType {
    /// Diff snapshot.
    Diff,
    /// Full snapshot.
    Full,
}

impl Default for SnapshotType {
    fn default() -> SnapshotType {
        SnapshotType::Full
    }
}

/// Stores the configuration that will be used for creating a snapshot.
#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CreateSnapshotParams {
    /// This marks the type of snapshot we want to create.
    /// The default value is `Full`, which means a full snapshot.
    #[serde(default = "SnapshotType::default")]
    pub snapshot_type: SnapshotType,
    /// Path to the file that will contain the microVM state.
    pub snapshot_path: PathBuf,
    /// Path to the file that will contain the guest memory.
    pub mem_file_path: PathBuf,
    /// Optional field for the microVM version. The default
    /// value is the current version.
    pub version: Option<String>,
}

/// Stores the configuration that will be used for loading a snapshot.
#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LoadSnapshotParams {
    /// Path to the file that contains the microVM state to be loaded.
    pub snapshot_path: PathBuf,
    /// Path to the file that contains the guest memory to be loaded.
    pub mem_file_path: PathBuf,
    /// Setting this flag will enable KVM dirty page tracking and will
    /// allow taking subsequent incremental snapshots.
    pub enable_diff_snapshots: bool,
    /// Setting this flag enables user page faults handling by a different process.
    pub enable_user_page_faults: bool,
    /// Path to the passfd socket.
    pub sock_file_path: PathBuf,
    /// overlay path
    pub overlay_file_path: PathBuf,
    /// Enable overlay regions mmap
    pub overlay_regions: HashMap<i64, i64>,
    /// ws file path
    pub ws_file_path: PathBuf,
    /// ws file mappings: 
    pub ws_regions: Vec<Vec<i64>>,
    /// enable locally load ws
    pub load_ws: bool,
    #[serde(default)]
    /// fadvise for memfile
    pub fadvise: String,
}

/// The microVM state options.
#[derive(Debug, Deserialize, Serialize)]
pub enum VmState {
    /// The microVM is paused, which means that we can create a snapshot of it.
    Paused,
    /// The microVM is resumed; this state should be set after we load a snapshot.
    Resumed,
}

/// Keeps the microVM state necessary in the snapshotting context.
#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Vm {
    /// The microVM state, which can be `paused` or `resumed`.
    pub state: VmState,
}
