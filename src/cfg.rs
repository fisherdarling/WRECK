use crate::production::Production;
use crate::symbol::{NonTerminal, Symbol, Terminal};
use anyhow::{anyhow, Result};
use std::collections::{BTreeMap, BTreeSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

#[derive(Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CFG {
    pub terminals: BTreeSet<Terminal>,
    pub non_terminals: BTreeSet<NonTerminal>,
    pub start_symbol: NonTerminal,
    pub production_map: BTreeMap<NonTerminal, Vec<usize>>,
    pub productions: Vec<Production>,
}

impl CFG {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn first(symbol: &Symbol) -> BTreeSet<Terminal> {
        todo!()
    }

    pub fn from_file(path: impl AsRef<Path>) -> Result<Self> {
        let mut cfg = CFG::new();

        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let mut lines = reader.lines().flatten().filter(|l| !l.is_empty());

        // Get the start symbol and its first production
        let (start, first_production) = lines
            .next()
            .ok_or(anyhow!("Cannot create an empty CFG"))
            .map(Line::from_str)??
            .into_start()?;

        // Add the first production to the list of productions
        let first_production: Production = first_production.into();
        cfg.productions.push(first_production);
        let production_idx = cfg.productions.len() - 1;

        // Add the first production to the start symbol
        cfg.production_map
            .entry(start.clone())
            .or_default()
            .push(production_idx);

        // Set the CFG start symbol
        cfg.start_symbol = start.clone();

        let mut current_nt = start;
        while let Some(Ok(line)) = lines.next().map(Line::from_str) {
            match line {
                Line::Start(new_nt, new_productions) => {
                    let production: Production = new_productions.into();
                    cfg.productions.push(production);
                    let production_idx = cfg.productions.len() - 1;

                    cfg.production_map
                        .entry(new_nt.clone())
                        .or_default()
                        .push(production_idx);

                    cfg.non_terminals.insert(new_nt.clone());

                    current_nt = new_nt;
                }
                Line::Union(new_productions) => {
                    for symbol in &new_productions {
                        if symbol.terminal().is_ok() {
                            cfg.terminals.insert(symbol.terminal().unwrap().clone());
                        }
                    }

                    let production: Production = new_productions.into();
                    cfg.productions.push(production);
                    let production_idx = cfg.productions.len() - 1;

                    cfg.production_map
                        .entry(current_nt.clone())
                        .or_default()
                        .push(production_idx);
                }
            }
        }

        Ok(cfg)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Line {
    Start(NonTerminal, Vec<Symbol>),
    Union(Vec<Symbol>),
}

impl Line {
    fn into_start(self) -> Result<(NonTerminal, Vec<Symbol>)> {
        if let Line::Start(nt, symbols) = self {
            Ok((nt, symbols))
        } else {
            Err(anyhow!("Line is not a Start: {:?}", self))
        }
    }

    fn into_union(self) -> Result<Vec<Symbol>> {
        if let Line::Union(symbols) = self {
            Ok(symbols)
        } else {
            Err(anyhow!("Line is not a Union: {:?}", self))
        }
    }

    pub fn from_str(input: String) -> Result<Line> {
        let mut split: Vec<&str> = input.trim().split(' ').collect();

        match &split.as_slice() {
            [nt, "->", symbols @ ..] => {
                let nt = Symbol::from_parse(nt)?.non_terminal()?.clone();
                let symbols = symbols
                    .iter()
                    .copied()
                    .map(Symbol::from_parse)
                    .collect::<Result<Vec<Symbol>>>()?;

                Ok(Line::Start(nt, symbols))
            }
            ["|", symbols @ ..] => {
                let symbols = symbols
                    .iter()
                    .copied()
                    .map(Symbol::from_parse)
                    .collect::<Result<Vec<Symbol>>>()?;

                Ok(Line::Union(symbols))
            }

            _ => panic!(format!("Split: {:?}", split)),
        }
    }
}
