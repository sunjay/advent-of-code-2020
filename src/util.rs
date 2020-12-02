use std::{io::{self, BufRead}, fs::File, path::Path, str::FromStr};

use anyhow::Context;

pub fn read_lines<T>(path: &Path) -> anyhow::Result<Vec<T>>
    where T: FromStr,
          <T as FromStr>::Err: std::error::Error + Send + Sync + 'static,
{
    let file = File::open(&path)
        .with_context(|| format!("Failed to read `{}`", path.display()))?;
    let reader = io::BufReader::new(file);

    let mut items = Vec::new();
    for line in reader.lines() {
        let line = line.with_context(|| {
            format!("Failed to read line from `{}`", path.display())
        })?;
        let item = line.parse().with_context(|| {
            format!("Failed to parse line from `{}`", path.display())
        })?;
        items.push(item);
    }

    Ok(items)
}
