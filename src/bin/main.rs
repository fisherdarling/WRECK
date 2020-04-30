#![allow(non_snake_case)]

use wreck::cfg::CFG;

fn main() -> anyhow::Result<()> {
    let cfg = CFG::from_file("llre.cfg");

    println!("{:?}", cfg);

    Ok(())
}
