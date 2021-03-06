#![allow(non_snake_case)]

use wreck::cfg::CFG;
use wreck::input::LexerConfig;
use wreck::ll_table::LLTable;
use wreck::nfa_generator::NFAGenerator;
use wreck::parser::Parser;

use std::collections::BTreeSet;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, Clone, StructOpt)]
pub struct Args {
    #[structopt(parse(from_os_str))]
    input: PathBuf,
    #[structopt(parse(from_os_str))]
    output: PathBuf,
}

fn main() {
    let args = Args::from_args();

    let config = LexerConfig::from_file(args.input);

    println!("Alphabet: {:?}\n", config.alphabet);

    glue(&config, args.output);
}

// TODO this should probably be moved to main, just doing it here so we don't get merge conflicts
fn glue(config: &LexerConfig, output: impl AsRef<std::path::Path>) {
    let mut output = File::create(output).unwrap();
    write_alphabet(&mut output, &config.alphabet);

    let cfg = CFG::from_file("llre.cfg").unwrap(); // TODO this is the only input, right?
    let table = LLTable::from_cfg(&cfg);
    for input_line in &config.regexes {
        println!("working on {}", input_line.1);
        let mut lexer = silly_lex::Lexer::new(&input_line.0).iter();
        let mut parser = Parser::new(&cfg, &table);
        let tree = parser.parse(&mut lexer.peekable());

        let mut dot_output = input_line.1.clone();
        dot_output.push_str(".dot");
        tree.export_graph(&dot_output);

        let simplified = wreck::ast::simplify_RE(&tree);

        let mut simplified_dot_output = input_line.1.clone();
        simplified_dot_output.push_str("_simple.dot");

        simplified.export_graph(&simplified_dot_output);

        let mut generator = NFAGenerator::new(config.alphabet.clone(), Some(input_line.1.clone()));
        // TODO these could probably be mixed together into a single 'generate' command
        generator.add_to_table(&simplified, 0, 1);
        generator.create_nfa().unwrap();

        writeln!(
            output,
            "{}.tt\t{}\t{}",
            input_line.1,
            input_line.1,
            input_line.2.as_deref().unwrap_or("")
        )
        .unwrap();
    }

    output.flush().unwrap();

    // let simplified = wreck::ast::simplify_RE(&tree);
}

fn write_alphabet(out: &mut dyn Write, alpha: &BTreeSet<char>) {
    for c in alpha {
        write!(out, "x{:02X}", *c as u8).unwrap();
    }

    writeln!(out).unwrap();
}

fn print_table(cfg: &CFG, lltable: &LLTable) {
    println!("Productions:");

    for (i, production) in cfg.productions.iter().enumerate() {
        let nt = cfg
            .production_map
            .iter()
            .find(|(_, v)| v.contains(&i))
            .map(|(k, _)| k)
            .expect("A production must have a left hand side.");

        println!("[{:>2}] {: <8} -> {}", i, nt.non_terminal(), production);
    }

    println!();

    println!("Table:");

    print!("{: <10}", "");
    let (_, row) = lltable.table.iter().next().unwrap();

    for (terminal, _) in row.iter() {
        print!("{: >8}", terminal.terminal());
    }

    println!();

    for (nt, row) in lltable.table.iter() {
        print!("{: <10}", nt.non_terminal());

        for (_, transition) in row.iter() {
            if let Some(production) = transition {
                print!("{: >8}", production)
            } else {
                print!("{: >8}", "-");
            }
        }

        println!();
    }
    // for nt in &cfg.non_terminals {

    //     for terminal in &cfg.terminals {
    //         if let Some(production) = lltable.table[nt][terminal] {
    //             print!("{: >8}", production)
    //         } else {
    //             print!("{: >8}", ".");
    //         }
    //     }

    //     println!();
    // }

    println!();
}

// fn main() -> anyhow::Result<()> {
//     let cfg = CFG::from_file("llre.cfg").unwrap();

//     // println!("{:#?}", cfg);

//     // // for nt in &cfg.non_terminals {
//     // //     println!(
//     // //         "{:?}: {:?}",
//     // //         nt,
//     // //         cfg.derives_to_lambda(&nt, &mut Vec::new())
//     // //     );
//     // // }

//     println!("First Sets:");
//     for nt in &cfg.non_terminals {
//         println!("{: <8}: {:?}", nt.non_terminal(), cfg.first_set(&nt));
//     }

//     println!();

//     println!("Follow Sets:");
//     for nt in &cfg.non_terminals {
//         println!(
//             "{: <8}: {:?}",
//             nt.non_terminal(),
//             cfg.follow(&nt, Default::default()).0
//         );
//     }

//     println!();

//     println!("Predict Sets:");
//     for (i, production) in cfg.productions.iter().enumerate() {
//         let nt = cfg
//             .production_map
//             .iter()
//             .find(|(_, v)| v.contains(&i))
//             .map(|(k, _)| k)
//             .expect("A production must have a left hand side.");

//         println!(
//             "[{:>2}] {: <8} -> {:?}",
//             i,
//             nt.non_terminal(),
//             cfg.predict_set(nt, production)
//         );
//     }
//     // for nt in &cfg.non_terminals {
//     //     for production_index in &cfg.production_map[nt] {
//     //         let production = &cfg.productions[*production_index];
//     //         let predict_set = cfg.predict_set(nt, production);

//     //         println!("{: <8}: {:?}", nt.non_terminal(), predict_set);
//     //     }
//     // }

//     println!();

//     let table = LLTable::from_cfg(&cfg);
//     print_table(&cfg, &table);

//     println!();
//     println!();
//     println!();

//     let regex = "Ab(cd-e+)*(.|012)3";
//     println!("Parsing Regex: {}", regex);

//     let mut lexer = silly_lex::Lexer::new(&regex).iter();

//     let mut parser = Parser::new(&cfg, &table);
//     let tree = parser.parse(&mut lexer.peekable());

//     tree.export_graph("regex.dot");
//     let simplified = wreck::ast::simplify_RE(&tree);
//     simplified.export_graph("regex_simpl.dot");

//     Ok(())
// }
