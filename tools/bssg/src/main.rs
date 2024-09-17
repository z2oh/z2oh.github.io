#![allow(dead_code)]
#![allow(unreachable_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use std::{ffi::OsStr, path::{Path, PathBuf}};
use anyhow::{Context, Result};

mod builder;
mod util;

fn main() -> Result<()> {
    if cfg!(debug_assertions) {
        colog::basic_builder().filter_level(log::LevelFilter::Trace).init();
    } else {
        colog::init();
    };

    let www_path = std::path::absolute("../../www")?;
    let out_path = std::path::absolute("../../workdir/www_out")?;

    let builder = builder::Builder::new(www_path, out_path)?;
    builder.build()?;

    Ok(())
}