#![allow(dead_code)]

use indexmap::IndexMap;
use rand::prelude::*;

pub struct ImplicationInformation {
    pub literal: u32,
    pub implied_by_vars: Vec<u32>,
    pub implied_by_clause: Vec<u32>
}

// pub struct ConflictInformation {
//     pub caused_by_clause: Vec<u32>

// }

pub struct CNFFormula {
    pub m_clauses: Vec<Vec<u32>>,
    pub m_variables: IndexMap<String, u32>,
    pub m_assignments: IndexMap<String, bool>,
    pub m_decision_level: i32,
    pub m_implications: Vec<Vec<ImplicationInformation>>
}

impl CNFFormula {
    pub fn add_clause(variables: &IndexMap<String, u32>, clause: Vec<String>) -> Vec<u32> {
        let mut literals: Vec<u32> = Vec::new();

        println!("Clause: {:?}", clause);

        for literal in clause {
            if literal.starts_with("-") {
                literals.push(variables.get(&literal.trim_matches('-').to_owned()).unwrap().clone() << 1 | 1)
            }
            else if !literal.starts_with("0") {
                literals.push(variables.get(&literal).unwrap().clone() << 1)
            }
        }        

        //println!("Literals: {:?}", literals);
        return literals;
    }

    pub fn add_conflict_cause(&mut self, literals: Vec<u32>) {
        let mut literals_negation: Vec<u32> = Vec::new();
        for literal in &literals {
            literals_negation.push(literal ^ 1);
        }

        println!("Adding clause {:?} -> {:?}", literals, literals_negation);
    }

    pub fn new(clause_pile: Vec<Vec<String>>) -> CNFFormula {
        let mut clauses: Vec<Vec<u32>> = Vec::new();
        let mut variables: IndexMap<String, u32> = IndexMap::new();

        let mut var_index = 0;

        for clause in clause_pile {
            for literal in clause.clone() {
                if literal.starts_with("-") {
                    if !variables.contains_key(&literal.trim_matches('-').to_owned()) {
                        variables.insert(literal.trim_matches('-').to_owned(), var_index);
                        var_index += 1;
                    }
                }
                else if !literal.starts_with("0") {
                        if !variables.contains_key(&literal) {
                        variables.insert(literal, var_index);
                        var_index += 1;
                    }
                }
            }
            clauses.push(Self::add_clause(&variables, clause));
        }


        println!("Variables: {:?}\n", variables);

        // Sort strings as if they were integers.
        // variables.sort_unstable_by(|a: &String, b: &String| a.parse::<i32>().unwrap().cmp(&b.parse::<i32>().unwrap()));

        // Delete duplicates.
        // variables.dedup();
 
        return CNFFormula{ m_clauses: clauses , m_variables: variables, 
                           m_assignments: IndexMap::new(), m_decision_level: -1,
                           m_implications: Vec::new() };
    } 

    pub fn lit_to_string(&self, literal: u32) -> String {
        let mut negated = false;
        if literal & 1 != 0 {
            negated = true;
        }

        let mut num = self.m_variables.get_index((literal >> 1) as usize)
                                    .unwrap()
                                    .0
                                    .to_string();

        if negated {
            num.insert(0, '-');
        }

        return num;
    }

    pub fn lit_list_to_strings(&self, literals: Vec<u32>) -> Vec<String> {
        let mut strings = Vec::new();
        for literal in literals {
            let mut negated = false;
            if literal & 1 != 0 {
                negated = true;
            }

            let mut num = self.m_variables.get_index((literal >> 1) as usize)
                                        .unwrap()
                                        .0
                                        .to_string();

            if negated {
                num.insert(0, '-');
            }
            strings.push(num);
        }

        return strings;
    }

    pub fn print_implications(&self, imp_inf: &ImplicationInformation) {
        println!("variable: {:?} ", self.lit_to_string(imp_inf.literal));
        println!("Implied by vars -> {:?}", self.lit_list_to_strings(imp_inf.implied_by_vars.clone()));
        println!("Implied by clause -> {:?}", self.lit_list_to_strings(imp_inf.implied_by_clause.clone()));
    }

    pub fn print_current_level_implications(&self) {
        for imp_info in &self.m_implications[self.m_decision_level as usize] {
            self.print_implications(&imp_info);
        }
    }

    pub fn is_finished(&self) -> bool {
        if self.m_assignments.len() == self.m_variables.len() {
            println!("Solution:\n{:?}", self.m_assignments);
            return true;
        } else {
            return false;
        }
    }

    pub fn analyze_conflict(&mut self, clause: &Vec<u32>) -> u32 {

        return 0;
    }

    pub fn make_decision(&mut self) {
        let mut rng = thread_rng();
        let mut index = rng.gen_range(0, self.m_variables.len());
        while self.m_assignments.contains_key(&self.m_variables.get_index(index)
                                                  .unwrap()
                                                  .0
                                                  .to_string()) {

            index = rng.gen_range(0, self.m_variables.len());
        }
        
        let rnd_bool = rng.gen();

        self.m_assignments.insert(self.m_variables.get_index(index)
                                                  .unwrap()
                                                  .0
                                                  .to_string(), rnd_bool);

        self.m_decision_level += 1;
        self.m_implications.push(Vec::new());
    }

    pub fn make_decision_fake(&mut self, decision: u32) {
        self.m_assignments.insert(self.m_variables.get_index(decision as usize)
                                                    .unwrap()
                                                    .0
                                                    .to_string(), true);

        self.m_decision_level += 1;
        self.m_implications.push(Vec::new());
    }


    // pub fn apply_unit_propagation(&mut self, clause: Vec<u32>) -> bool {

    // }

    pub fn solve(&mut self) -> bool {
        let mut literal_assignments: Vec<u32> = Vec::new();

        // For potential conflict
        let mut conflict = false;
        let mut literals_negation: Vec<u32> = Vec::new();


        // Convert m_assignments into literal assignments.
        for (key, value) in &self.m_assignments {
            let mut negate: u32 = 0;
            if !value {
                negate = 1;
            }

            literal_assignments.push(self.m_variables.get(key).unwrap() << 1 | negate);
        }

        // Process clauses.
        for clause in &self.m_clauses {
            let mut free_literals: Vec<u32> = Vec::new();
            let mut implication_literals: Vec<u32> = Vec::new();

            let mut currently_sat = false;

            for literal in clause {
                let mut free = true;          
                for lit_assignment in &literal_assignments {
                    // If literal is free, push to vector.
                    if *literal == (lit_assignment ^ 1) {
                        free = false;

                        implication_literals.push(*literal ^ 1);
                    }
                    if literal == lit_assignment {
                        currently_sat = true;

                        implication_literals.push(*literal ^ 1);
                    }
                }
                if free {
                    free_literals.push(*literal);
                }
            }
            
            // pub struct Propagated_Information {
            //     pub variable: String,
            //     pub literal: u32,
            //     pub implied_by_vars: Vec<u32>,
            //     pub implied_by_clause: Vec<u32>
            // }

            // Now propagate
            if free_literals.len() == 1 && !currently_sat {

                let implication_info = ImplicationInformation {
                    literal: free_literals[0],
                    implied_by_vars: implication_literals,
                    implied_by_clause: clause.clone()
                };

                //self.print_implications(&implication_info);
                self.m_implications[self.m_decision_level as usize].push(implication_info);

                let mut negated = false;
                if free_literals[0] & 1 == 0 {
                    negated = true;
                }

                // Insert into assignments, the unit which must be true in order for clause to be true.
                self.m_assignments.insert(self.m_variables.get_index((free_literals[0] >> 1) as usize)
                                            .unwrap()
                                            .0
                                            .to_string(), negated);

                return false;
            }

            //println!("{} - {} - {:?}", free_literals.len(), currently_sat, clause);

            //If there is a conflict, add the clause
            if free_literals.len() == 0 && !currently_sat {
                println!("Conflict! CAUSED BY:");
                println!("{:?}", self.lit_list_to_strings(clause.clone()));

                self.analyze_conflict(&clause);

                let mut partial_learned_clause = clause.clone();

                self.print_current_level_implications();

                println!("SÅ KOMMER DER SIDST IMPLIED CLAUSES HER:\n\n");
                // FIX
                while let Some(last_impl) = self.m_implications[self.m_decision_level as usize].pop() {
                    self.print_implications(&last_impl);
                }

                println!("Variables:\n{:?}", self.m_variables);
                println!("Assignments:\n{:?}", self.m_assignments);

                // Create negation of literals, push onto m_clauses.
                for literal in clause {
                    literals_negation.push(*literal ^ 1);
                }
                //println!("Adding clause {:?} -> {:?}", clause, literals_negation);
                
                conflict = true;
                break;
                // GO BACK TO PREVIOUS CONFLICT LEVEL
            }
        }


        if conflict {
            // Push conflict clause.
            self.m_clauses.push(literals_negation);
            self.m_assignments.clear();
        }
        return true;
    }
}