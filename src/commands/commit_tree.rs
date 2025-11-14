use crate::objects::{Kind, Object};
use anyhow::Context;
use std::fmt::Write;
use std::io::Cursor;

pub(crate) fn invoke(
    message: String,
    tree_hash: String,
    parent_hash: Option<String>,
) -> anyhow::Result<()> {
    let mut commit = String::new();
    writeln!(commit, "tree {tree_hash}")?;
    if let Some(parent_hash) = parent_hash {
        writeln!(commit, "parent {parent_hash}")?;
    }
    writeln!(
        commit,
        "author 7h3w1zard <93vrs93@gmail.com> 1762976870 +0300"
    )?;
    writeln!(
        commit,
        "committer 7h3w1zard <93vrs93@gmail.com> 1762976870 +0300"
    )?;
    writeln!(commit, "")?;
    writeln!(commit, "{message}")?;

    let hash = Object {
        kind: Kind::Commit,
        expected_size: commit.len() as u64,
        reader: Cursor::new(commit),
    }
    .write_to_objects()
    .context("write commit object")?;

    println!("{}", hex::encode(hash));

    Ok(())
}
