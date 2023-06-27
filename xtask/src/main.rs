use std::env;
use std::ffi::OsStr;
use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;

use anyhow::{anyhow, Result};
use tracing::info;
use tracing::metadata::LevelFilter;

fn main() -> Result<()> {
    dotenv::dotenv().ok();

    tracing_subscriber::fmt()
        .with_max_level(LevelFilter::TRACE)
        .with_thread_ids(true)
        .with_file(true)
        .with_line_number(true)
        .with_thread_names(true)
        .with_ansi(true)
        .init();

    let task = env::args().nth(1);
    match task.as_deref() {
        Some("help") => print_help(),
        Some("codegen") => codegen()?,
        Some("dist") => dist()?,
        _ => print_help(),
    }

    Ok(())
}

// Tasks

fn print_help() {
    eprintln!(
        r#"
Tasks:

dist .......... build distribution artifacts
codegen ....... generate Rust code (needs erdb and quicktype)
help .......... print this help
"#
    );
}

fn codegen() -> Result<()> {
    const SCHEMAS: [&str; 17] = [
        "ammo",
        "armaments",
        "armor",
        "ashes-of-war",
        "bolstering-materials",
        "correction-attack",
        "correction-graph",
        "crafting-materials",
        "gestures",
        "info",
        "keys",
        "reinforcements",
        "shop",
        "spells",
        "spirit-ashes",
        "talismans",
        "tools",
    ];

    for schema in SCHEMAS {
        info!("Generating schema for {schema}...");
        let schema_code = run_cmd([
            "quicktype",
            "-l",
            "rust",
            "-s",
            "schema",
            schema_file(schema).to_string_lossy().as_ref(),
            "--density",
            "dense",
            "--derive-debug",
            "--derive-clone",
        ])?;

        fs::write(schema_type_file(schema), schema_code)?;
    }

    let mut mod_file = File::create(schema_type_file("mod"))?;
    for schema in SCHEMAS {
        writeln!(mod_file, "pub mod {};", schema.replace("-", "_"))?;
    }

    Ok(())
}

fn dist() -> Result<()> {
    Err(anyhow!("TODO"))
}

// Utils

fn run_cmd<I: IntoIterator<Item = S>, S: AsRef<OsStr>>(args: I) -> Result<String> {
    let mut args = args.into_iter();
    let cmd_name = args.next().expect("At least one argument needed");
    let output = Command::new(cmd_name).args(args).output()?;
    Ok(String::from_utf8(output.stdout)?)
}

fn project_root() -> PathBuf {
    Path::new(&env!("CARGO_MANIFEST_DIR")).ancestors().nth(1).unwrap().to_path_buf()
}

fn dist_dir() -> PathBuf {
    project_root().join("target/dist")
}

fn codegen_dir() -> PathBuf {
    project_root().join("src/codegen")
}

fn schema_file(schema: &str) -> PathBuf {
    project_root().join("data/schemas").join(format!("{schema}.schema.json"))
}

fn schema_type_file(schema: &str) -> PathBuf {
    codegen_dir().join(format!("{}.rs", schema.replace("-", "_")))
}
