use std::path::PathBuf;

use argh::FromArgs;
use redscript_io::{Definition, ScriptBundle};
use vmap::Map;

/// REDscript cache compact tool
#[derive(FromArgs)]
struct Opts {
    /// input cahe file path
    #[argh(option, short = 'i')]
    input: PathBuf,
    /// output cache file path
    #[argh(option, short = 'o')]
    output: PathBuf,
}

fn main() -> anyhow::Result<()> {
    let opts = argh::from_env::<Opts>();
    let (map, _file) = Map::with_options().open(opts.input)?;
    let mut bundle = ScriptBundle::from_bytes(&map)?;

    for def in bundle.definitions_mut() {
        match def {
            Definition::Function(function) => function.set_code(vec![]),
            // replace redundant definitions with bitfields
            Definition::Local(_) => *def = Definition::Bitfield,
            Definition::SourceFile(_) => *def = Definition::Bitfield,
            _ => {}
        }
    }

    bundle.strings_mut().clear();
    bundle.resources_mut().clear();
    bundle.tdb_ids_mut().clear();

    bundle.into_writeable().save(opts.output)?;
    Ok(())
}
