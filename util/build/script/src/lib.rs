// Copyright (c) 2018-2020 MobileCoin Inc.

#![feature(external_doc)]
#![doc(include = "../README.md")]

mod cargo_build;
mod env;
mod utils;
mod vars;

pub use self::{
    cargo_build::CargoBuilder,
    env::{
        Endianness, EndiannessError, Environment, EnvironmentError, TargetFamily, TargetFamilyError,
    },
    utils::rerun_if_path_changed,
};
