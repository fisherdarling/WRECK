use std::collections::BTreeMap;

use crate::{
    cfg::CFG,
    symbol::{NonTerminal, Symbol, Terminal},
};

pub type ProductionIndex = usize;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LLTable<'cfg> {
    pub table: BTreeMap<&'cfg NonTerminal, BTreeMap<Terminal, Option<usize>>>,
}

impl<'cfg> LLTable<'cfg> {
    pub fn from_cfg(cfg: &'cfg CFG) -> Self {
        let mut table = BTreeMap::new();

        let mut empty_map: BTreeMap<Terminal, Option<usize>> =
            cfg.terminals.iter().map(|t| (t.clone(), None)).collect();

        empty_map.insert(Terminal::from(String::from("$")), None);

        for nt in &cfg.non_terminals {
            table.insert(nt, empty_map.clone());
        }

        for nt in &cfg.non_terminals {
            for production_index in &cfg.production_map[nt] {
                let production = &cfg.productions[*production_index];
                let predict_set = cfg.predict_set(nt, production);

                // println!(
                //     "Predict Set: {} -> {}\n{:?}",
                //     nt.non_terminal(),
                //     production,
                //     predict_set
                // );

                for terminal in predict_set {
                    table
                        .entry(nt)
                        // .and_modify(|f| {
                        //     // println!("{:?} {:?}", terminal, f);
                        //     // if f[&terminal].is_some() {
                        //     //     panic!(format!("Collision! {:?}", f));
                        //     // }
                        // })
                        .or_default()
                        .insert(terminal.clone(), Some(*production_index));

                    // println!("After: {} => {:?}", terminal.terminal(), table[&nt]);
                }
            }
        }

        Self { table }
    }
}
