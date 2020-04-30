#![allow(non_snake_case)]

use wreck::cfg::CFG;

fn main() -> anyhow::Result<()> {
    let cfg = CFG::from_file("llre.cfg").unwrap();

    println!("{:#?}", cfg);

    for nt in &cfg.non_terminals {
        println!("{:?}: {:?}", nt, cfg.first_set(&nt));
    }

    // println!("{:#?}", cfg);

    Ok(())
}
