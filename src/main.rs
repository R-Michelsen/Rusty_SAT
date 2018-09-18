extern crate clap;
extern crate time;
extern crate indexmap;
extern crate rand;

use clap::{App, Arg};
use time::PreciseTime;

use std::fs::File;
use std::io::{BufRead, BufReader, Result};

mod cnf_formula;
use cnf_formula::CNFFormula;

fn main() -> Result<()> {
    let matches = App::new("Rusty SAT")
                    .arg(Arg::with_name("file")
                    .help("CNF formula as .cnf file")
                    .takes_value(true)
                    .short("f")
                    .long("file")
                    .required(true))
                    .get_matches();

    let file = File::open(matches.value_of("file").unwrap())?;
    
    let mut clause_pile: Vec<Vec<String>> = Vec::new();

    for line in BufReader::new(file).lines() {
        let line = line
                    .unwrap()
                    .trim_left()
                    .to_owned();
        
        if line.starts_with("p") {
            let cnf_declaration: Vec<String> = line
                                            .split_whitespace()
                                            .map(|string| string.to_owned())
                                            .collect();

            assert_eq!(cnf_declaration.len(), 4, "Couldn't parse file, corrupt file or incompatible format.");
            println!("Processing CNF Formula with {} variables and {} clauses",
                    cnf_declaration[2], cnf_declaration[3]);
        }

        else if (line.starts_with(char::is_numeric) || line.starts_with("-")) && !line.starts_with("0")  {
            let clause: Vec<String> = line
                                    .split_whitespace()
                                    .map(|string| string.to_owned())
                                    .collect();
            clause_pile.push(clause);
        }
    }

    let start = PreciseTime::now();

    // GO BACK TO FAKE SHIT AND FIGURE OUT WHY UR SHIT DOESNT WORK

    let mut cnf_formula = CNFFormula::new(clause_pile.clone());

    // cnf_formula.make_decision_fake(14, true); //11
    // while !cnf_formula.solve() {}

    // cnf_formula.make_decision_fake(4, true); //2
    // while !cnf_formula.solve() {}

    // cnf_formula.make_decision_fake(15, true); //11
    // while !cnf_formula.solve() {}

    // cnf_formula.make_decision_fake(18, true); //2
    // while !cnf_formula.solve() {}

    // cnf_formula.make_decision_fake(11, true); //13
    // while !cnf_formula.solve() {}

    // cnf_formula.make_decision_fake(3, false);
    // while !cnf_formula.solve() {}

    // cnf_formula.make_decision_fake(6, false);
    // while !cnf_formula.solve() {}

    // cnf_formula.make_decision_fake(8, false);
    // while !cnf_formula.solve() {}

    // cnf_formula.make_decision_fake(0, false);
    // while !cnf_formula.solve() {}


    loop {
        //if cnf_formula.m_decide_count % 500 == 0 { cnf_formula.restart(); }
        //else { cnf_formula.make_decision(); }
        cnf_formula.make_decision();
        while !cnf_formula.solve() {}
        if cnf_formula.m_finished { break; }

       // if cnf_formula.m_decide_count % 500 == 0 { cnf_formula.print_stats(); }
    }

    let end = PreciseTime::now();
    println!("\nCNF Evaluated in {} seconds.", start.to(end).num_milliseconds() as f64 / 1000.0);
    Ok(())
}
