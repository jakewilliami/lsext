use clap::{crate_authors, crate_version, value_parser, ArgAction, Parser};
use ignore::WalkBuilder;
use std::{collections::HashMap, ffi::OsStr};
use tabular::{row, Table};

// TODO: Proportion (in percentage of bytes)
// TODO: `file` to get type of file rather than extension?
// TODO: aggregate less than value/summary (see dutree)

#[derive(Parser)]
#[command(name = "lsext", author = crate_authors!("\n"), version = crate_version!())]
/// Summary of files by extension
///
/// Recurse files from a directory, and list them by extension frequency.
struct Cli {
    /// Directory from which you wish to start counting (optional)
    ///
    /// Defaults to the current directory
    #[arg(
        action = ArgAction::Set,
        num_args = 1,
        value_parser = value_parser!(String),
        value_name = "dir",
    )]
    dir: Option<String>,

    /// Count all files
    ///
    /// By default, the programme doesn't recurse into hidden/ignored directories.  This switch ensures all files are counted.
    #[arg(
        short = 'a',
        long = "all",
        action = ArgAction::SetTrue,
        num_args = 0,
    )]
    all: Option<bool>,
}

fn main() {
    let cli = Cli::parse();

    // Get directory, or default
    let dir = if let Some(dir) = cli.dir {
        dir
    } else {
        String::from(".")
    };

    let mut ext_freqs = HashMap::new();

    // let walk_f = walkdir::WalkDir::new(".")
    let walk_f = WalkBuilder::new(dir)
        .standard_filters(!cli.all.unwrap())
        .build();

    // Find files
    for e in walk_f.into_iter().filter_map(|e| e.ok()) {
        if e.metadata().unwrap().is_file() {
            // Get extension of file (if present)
            let no_ext_key = OsStr::new("<no extension>").to_owned();
            let ext = if let Some(ext) = e.path().extension() {
                if ext.is_empty() {
                    no_ext_key
                } else {
                    ext.to_owned()
                }
            } else {
                no_ext_key
            };

            // Add to counter
            ext_freqs.entry(ext).and_modify(|v| *v += 1).or_insert(1);
        }
    }

    // Sort extensions by frequency (and then in alphabetical order of extension)
    let mut ext_freqs: Vec<_> = ext_freqs.iter().collect();
    ext_freqs.sort_by(|a, b| {
        let num_order = b.1.cmp(a.1);
        num_order.then_with(|| a.0.cmp(b.0))
    });

    // Display the count map of extensions
    let mut table = Table::new("{:>}  {:<}");
    for (ext, freq) in ext_freqs.iter() {
        table.add_row(row!(freq, ext.to_str().unwrap()));
    }
    print!("{}", table);
}
