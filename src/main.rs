use std::io::{BufWriter, Result, Write};

use cargo_lock::*;

fn write_to(o: &mut impl Write) -> Result<()> {
    let lock = Lockfile::load("Cargo.lock").unwrap();

    writeln!(o, "CRATES=\"")?;

    for p in lock.packages {
        // local deps
        if p.source.is_none() {
            continue;
        }

        writeln!(o, "{}-{}", p.name, p.version)?;
    }

    writeln!(o, "\"")?;

    Ok(())
}

fn main() -> Result<()> {
    let mut args = pico_args::Arguments::from_env();

    if let Ok(arg) = args.free_from_str::<std::path::PathBuf>() {
        let mut text = std::fs::read_to_string(&arg)?;

        const CRATES: &str = "CRATES=\"\n";

        if let Some((start, end)) = text.find(CRATES).and_then(|start| {
            text[start + CRATES.len()..]
                .find("\"\n")
                .map(|end| (start, start + CRATES.len() + 2 + end))
        }) {
            let mut out = Vec::new();
            write_to(&mut out)?;
            text.replace_range(start..end, std::str::from_utf8(&out).unwrap());
            std::fs::write(arg, text)?;
        } else {
            write_to(&mut BufWriter::new(std::fs::File::open(&arg)?))?;
        }
    } else {
        let out = std::io::stdout();
        write_to(&mut out.lock())?;
    }

    Ok(())
}
