use std::{io::{self, BufRead}, fs::File, path::PathBuf, str::FromStr};

use anyhow::Context;

pub fn read_lines<T>(path: PathBuf) -> anyhow::Result<impl Iterator<Item=anyhow::Result<T>>>
    where T: FromStr,
          <T as FromStr>::Err: Into<anyhow::Error> + Send + Sync + 'static,
{
    let file = File::open(&path)
        .with_context(|| format!("Failed to read `{}`", path.display()))?;
    let reader = io::BufReader::new(file);

    Ok(reader.lines().map(move |line| -> anyhow::Result<_> {
        let line = line.with_context(|| {
            format!("Failed to read line from `{}`", path.display())
        })?;

        let item = line.parse().map_err(Into::into).with_context(|| {
            format!("Failed to parse line from `{}`", path.display())
        })?;

        Ok(item)
    }))
}
