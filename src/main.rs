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
    let start = PreciseTime::now();
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
            println!("Processing CNF Formula with {} variables and {} clauses\n",
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



    //loop {
        let mut cnf_formula = CNFFormula::new(clause_pile.clone());
        loop {
            cnf_formula.make_decision();
            while !cnf_formula.solve() {}
            if cnf_formula.m_finished { break; }
        }
    //}




    // cnf_formula.make_decision_fake(3, false);
    // cnf_formula.make_decision_fake(6, false);
    // cnf_formula.make_decision_fake(8, false);
    // cnf_formula.make_decision_fake(0, false);

    // loop {
    //     while !cnf_formula.solve() {}
    //     if cnf_formula.m_finished { break; }
    //     cnf_formula.make_decision();
    // }




    let end = PreciseTime::now();
    println!("\n\n{} seconds for whatever you did.", start.to(end));
    Ok(())
}
