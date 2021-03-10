use cargo_lock::*;

fn main() {
    let lock = Lockfile::load("Cargo.lock").unwrap();

    println!("CRATES=\"");

    for p in lock.packages {
        // local deps
        if p.source.is_none() {
            continue;
        }

        println!("{}-{}", p.name, p.version);
    }

    println!("\"");
}
