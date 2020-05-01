#![allow(non_snake_case)]

use wreck::cfg::CFG;
use wreck::input::LexerConfig;
use wreck::ll_table::LLTable;
use wreck::parser::Parser;

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

    glue(&config);
}

// TODO this should probably be moved to main, just doing it here so we don't get merge conflicts
fn glue(config: &LexerConfig) {
    let cfg = CFG::from_file("llre.cfg").unwrap(); // TODO this is the only input, right?
    let table = LLTable::from_cfg(&cfg);
    for input_line in &config.regexes {
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
    }

    // let simplified = wreck::ast::simplify_RE(&tree);
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
