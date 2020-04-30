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

    pub fn derives_to_lambda(
        &self,
        nt: &NonTerminal,
        stack: &mut Vec<(Production, NonTerminal)>,
    ) -> bool {
        for production_idx in &self.production_map[nt] {
            let production = &self.productions[*production_idx];

            if production.only_lambda() {
                return true;
            }

            if production.contains_terminal() {
                continue;
            }

            let mut all_derive_lambda = true;

            for rhs_symbol in production.symbols().iter() {
                let rhs = rhs_symbol.non_terminal().unwrap();
                let tuple = (production.clone(), rhs.clone());

                if stack.contains(&tuple) {
                    continue;
                }

                stack.push(tuple);
                all_derive_lambda = self.derives_to_lambda(rhs, stack);
                stack.pop();

                if !all_derive_lambda {
                    break;
                }
            }

            if all_derive_lambda {
                return true;
            }
        }

        false
    }

    pub fn first_set(&self, non_terminal: &NonTerminal) -> BTreeSet<Terminal> {
        let mut first_set = BTreeSet::new();

        for production_index in &self.production_map[non_terminal] {
            let production = &self.productions[*production_index];
            let (first, rest) = self.first(production.symbols(), BTreeSet::new());
            first_set.extend(first.into_iter());
        }

        first_set
    }

    pub fn first(
        &self,
        symbols: &[Symbol],
        mut t: BTreeSet<Symbol>,
    ) -> (BTreeSet<Terminal>, BTreeSet<Symbol>) {
        if let Some((symbol, rest)) = symbols.split_first() {
            // println!("{:?} . {:?}", symbol, rest);

            // The first set of a terminal is simply itself:
            if let Ok(terminal) = symbol.terminal() {
                let mut set = BTreeSet::new();
                set.insert(terminal.clone());
                return (set, t);
            }

            // The first of Lambda is empty:
            if symbol.is_lambda() {
                return (BTreeSet::new(), t);
            }

            let mut f: BTreeSet<Terminal> = BTreeSet::new();

            if !t.contains(&symbol) {
                t.insert(symbol.clone());

                // Find all lists of symbols that follow the usage of symbol
                for production in &self.productions {
                    // If this production contains the symbol:
                    if let Some(index) = production.index_of(&symbol) {
                        // The production is just of itself
                        if index + 1 == production.symbols().len() {
                            continue;
                        }

                        let rhs_symbol = &production[index + 1];

                        // Get the production for this non_terminal
                        if let Ok(rhs_nt) = rhs_symbol.non_terminal() {
                            for &rhs_production_index in &self.production_map[rhs_nt] {
                                let rhs_production = &self.productions[rhs_production_index];
                                let (g, _s) = self.first(rhs_production.symbols(), t.clone());
                                f.extend(g.into_iter());
                            }
                        }
                    }
                }
            }

            if self.derives_to_lambda(
                symbol
                    .non_terminal()
                    .expect("Cannot be a terminal at this point"),
                &mut Vec::new(),
            ) {
                let (g, _s) = self.first(rest, t.clone());
                f.extend(g.into_iter());
            }

            return (f, t);
        } else {
            return Default::default();
        }
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
        println!("From Str: {:?}", input);

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
