use anyhow::Context;
use std::fs::File;
use std::io::Read;
use std::path::Path;

use anyhow::Result;

use serde::Deserialize;

pub(crate) fn read_json_from_file<T>(filename: &Path) -> Result<T>
where
    for<'de> T: Deserialize<'de>,
{
    // Reading a file into a string before invoking Serde is faster than
    // invoking Serde from a BufReader, see
    // https://github.com/serde-rs/json/issues/160
    let mut string = String::new();
    File::open(filename)?.read_to_string(&mut string)?;
    serde_json::from_str(&string)
        .with_context(|| format!("Unable to parse JSON from file {:?}", filename))
}
