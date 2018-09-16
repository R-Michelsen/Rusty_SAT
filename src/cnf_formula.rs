#![allow(dead_code)]


use indexmap::IndexMap;
use rand::prelude::*;

pub struct ImplicationInformation {
    pub literal: u32,
    pub implied_by_vars: Vec<u32>,
    pub implied_by_clause: Vec<u32>,
}

pub struct CNFFormula {
    pub m_runs: u32,
    pub m_finished: bool,
    pub m_clauses: Vec<Vec<u32>>,
    pub m_variables: IndexMap<String, u32>,
    pub m_assignments: IndexMap<u32, i32>,
    pub m_saved_assignments: Vec<IndexMap<u32, i32>>,
    pub m_decision_level: i32,
    pub m_decision_guesses: IndexMap<u32, i32>,
    pub m_decision_level_implications: Vec<Vec<ImplicationInformation>>,
}

impl CNFFormula {
    pub fn add_clause(variables: &IndexMap<String, u32>, clause: Vec<String>) -> Vec<u32> {
        let mut literals: Vec<u32> = Vec::new();

        //println!("Clause: {:?}", clause);

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

    pub fn new(clause_pile: Vec<Vec<String>>) -> CNFFormula {
        let mut clauses: Vec<Vec<u32>> = Vec::new();
        let mut variables: IndexMap<String, u32> = IndexMap::new();

        let mut var_index = 0;

        for clause in clause_pile {
            println!("Clause: {:?}", clause);
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


        println!("Variables: {:?}", variables);


 
        return CNFFormula{ m_runs: 0, m_finished: false,
                           m_clauses: clauses , m_variables: variables,
                           m_saved_assignments: Vec::new(),
                           m_assignments: IndexMap::new(), m_decision_level: -1,
                           m_decision_guesses: IndexMap::new(),
                           m_decision_level_implications: Vec::new() };
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

    pub fn print_assignments(&self, assignments: &IndexMap<u32, i32>) {
        for (key, value) in assignments {
            print!("{}@{}, ", self.lit_to_string(key.clone()), value);
        }
        println!();
    }

    pub fn print_implications(&self, imp_inf: &ImplicationInformation) {
        println!("Variable: {} ", self.lit_to_string(imp_inf.literal));
        println!("Implied by vars -> {:?}", self.lit_list_to_strings(imp_inf.implied_by_vars.clone()));
        println!("Implied by clause -> {:?}", self.lit_list_to_strings(imp_inf.implied_by_clause.clone()));
    }

    pub fn print_current_level_implications(&self) {
        for imp_info in &self.m_decision_level_implications[self.m_decision_level as usize] {
            self.print_implications(&imp_info);
        }
    }

    pub fn update_partial_clause(partial_clause: &Vec<u32>, current_clause: &Vec<u32>) -> Vec<u32> {
        let mut new_partial_clause = Vec::new();

        for &partial_literal in partial_clause {
            if !current_clause.contains(&(partial_literal ^ 1)) {
                new_partial_clause.push(partial_literal);
            }
        }

        for &literal in current_clause {
            if !partial_clause.contains(&(literal ^ 1)) && !new_partial_clause.contains(&literal){
                new_partial_clause.push(literal);
            }
        }

        return new_partial_clause;
    }

    pub fn make_decision(&mut self) {        
        self.m_decision_level += 1;
        self.m_decision_level_implications.push(Vec::new());

        let mut rng = thread_rng();

        let mut index = rng.gen_range(0, self.m_variables.len());
        let negated = rng.gen_range(0, 2);

        while self.m_assignments.contains_key(&((self.m_variables.get_index(index).unwrap().1.clone() << 1) | 1)) ||
              self.m_assignments.contains_key(&(self.m_variables.get_index(index).unwrap().1.clone() << 1)) {
            index = rng.gen_range(0, self.m_variables.len());
        }

        self.m_decision_guesses.insert(*self.m_variables.get_index(index).unwrap().1 << 1 | negated, self.m_decision_level);
        self.m_assignments.insert(*self.m_variables.get_index(index).unwrap().1 << 1 | negated, self.m_decision_level);
        println!("Made decision {}", *self.m_variables.get_index(index).unwrap().1 << 1 | negated);
        

        // // Push decision literal.
        // if rnd_bool {
        //     self.m_decision_guesses.insert(*self.m_variables.get_index(index).unwrap().1 << 1, self.m_decision_level);
        //     self.m_assignments.push(*self.m_variables.get_index(index).unwrap().1 << 1);
        //     println!("Made decision {}", *self.m_variables.get_index(index).unwrap().1 << 1)
        // } else {
        //     self.m_decision_guesses.insert(*self.m_variables.get_index(index).unwrap().1 << 1 | 1, self.m_decision_level);
        //     self.m_assignments.push(*self.m_variables.get_index(index).unwrap().1 << 1 | 1);
        //     println!("Made decision {}", *self.m_variables.get_index(index).unwrap().1 << 1 | 1)
        // }

        // Save state.
        self.m_saved_assignments.push(self.m_assignments.clone());
   
    }

    pub fn make_decision_fake(&mut self, decision: u32, truthval: bool) {
        self.m_decision_level += 1;
        self.m_decision_level_implications.push(Vec::new());

        if truthval {
            self.m_decision_guesses.insert(*self.m_variables.get_index(decision as usize).unwrap().1 << 1 | 0, self.m_decision_level);
            self.m_assignments.insert(*self.m_variables.get_index(decision as usize).unwrap().1 << 1 | 0, self.m_decision_level);
            println!("Made decision {}", *self.m_variables.get_index(decision as usize).unwrap().1 << 1 | 0);
        } else {
            self.m_decision_guesses.insert(*self.m_variables.get_index(decision as usize).unwrap().1 << 1 | 1, self.m_decision_level);
            self.m_assignments.insert(*self.m_variables.get_index(decision as usize).unwrap().1 << 1 | 1, self.m_decision_level);
            println!("Made decision {}", *self.m_variables.get_index(decision as usize).unwrap().1 << 1 | 1);
        }


        // Save state.
        self.m_saved_assignments.push(self.m_assignments.clone());
    }

    pub fn solve(&mut self) -> bool {
        if (self.m_runs+1) % 500 == 0 {
            self.m_saved_assignments.clear();
            self.m_decision_guesses.clear();
            self.m_decision_level_implications.clear();
            self.m_decision_level = -1;
            self.m_assignments.clear();
            self.m_runs = 0;
        }

        //println!("CURRENT ASSIGNMENTS ----- {:?}", self.m_decision_guesses);
        //println!("CURRENT ASSIGNMENTS ----- {:?}", self.m_assignments);
        self.print_assignments(&self.m_assignments);

        //let mut literal_assignments: Vec<u32> = Vec::new();

        // For potential conflict
        let mut conflict = false;
        let mut conflict_clause: Vec<u32> = Vec::new();
        //let mut partial_learned_clause: Vec<u32> = Vec::new();
        let mut sat_count = 0;
        let mut bt_level: i32 = -1;

        // UIP var
        let mut uip = 0;
        
        
        //let mut propagate_count = 0;

        // Convert m_assignments into literal assignments.

        

        // for (key, value) in &self.m_assignments {
        //     let mut negate: u32 = 0;
        //     if !value {
        //         negate = 1;
        //     }

        //     literal_assignments.push(self.m_variables.get(key).unwrap() << 1 | negate);
        // }


        // Process clauses.
        for clause in &self.m_clauses {
            let mut free_literals: Vec<u32> = Vec::new();
            let mut implication_literals: Vec<u32> = Vec::new();
            let mut currently_sat = false;
            

            for literal in clause.clone() {
                let mut free = true;          
                for lit_assignment in self.m_assignments.keys() {        
                    if literal == (lit_assignment ^ 1) {
                        free = false;

                        // If unit propagation can be made, this array contains the contradicting literals 
                        // AKA the ones that imply the unit propagated.
                        implication_literals.push(literal ^ 1);
                    }
                    if literal == *lit_assignment {
                        currently_sat = true;
                    }
                }

                // If literal is free, push to vector.
                if free {
                    free_literals.push(literal);
                }     
            }

            // Now propagate
            if free_literals.len() == 1 && !currently_sat {
                let implication_info = ImplicationInformation {
                    literal: free_literals[0],
                    implied_by_vars: implication_literals,
                    implied_by_clause: clause.clone(),
                };

                self.print_implications(&implication_info);
                self.m_decision_level_implications[self.m_decision_level as usize].push(implication_info);
                self.m_assignments.insert(free_literals[0], self.m_decision_level);

                // let mut negated = false;
                // if free_literals[0] & 1 == 0 {
                //     negated = true;
                // }

                // // Insert into assignments, the unit which must be true in order for clause to be true.
                // self.m_assignments.insert(self.m_variables.get_index((free_literals[0] >> 1) as usize)
                //                             .unwrap()
                //                             .0
                //                             .to_string(), negated);
                return false;
            }

            //If there is a conflict, add the clause
            if free_literals.len() == 0 && !currently_sat {
                conflict = true;

                // Start the partial learned clause by cloning the current clause.
                println!("Conflict! {:?} @ decision_level = {}", self.lit_list_to_strings(clause.clone()), self.m_decision_level);

                // FIND CLAUSE TO LEARN.

                // Find UIP
                conflict_clause = clause.clone();
                //while let Some(last_impl) = self.m_decision_level_implications[self.m_decision_level as usize].pop() {
                
                for last_impl in self.m_decision_level_implications[self.m_decision_level as usize].iter().rev() {
                    conflict_clause = Self::update_partial_clause(&conflict_clause, &last_impl.implied_by_clause);

                    // Count literals at current decision level that are in clause. (Also find UIP).
                    let mut lit_count = 0;
                    let mut potential_uip = 0;
                    for p_lit in &conflict_clause {
                        for (key, value) in self.m_assignments.iter() {
                            if key == &(p_lit^1) && value == &self.m_decision_level {
                                potential_uip = p_lit ^ 1;
                                lit_count += 1;
                            }
                        }

                        // Calculate amount of assignments. are at current level.
                        // let mut count = self.m_assignments.len();
                        // if self.m_decision_level > 0 {
                        //     count = self.m_assignments.len() - self.m_saved_assignments[(self.m_decision_level - 1) as usize].len();
                        // }

                        // // Check how many literals left at current decision level
                        // for x in 0..count {
                        //     //println!("assignment -> {:?}", self.lit_to_string(self.m_assignments[self.m_assignments.len() - x - 1]));

                        //     if self.m_assignments[self.m_assignments.len() - x - 1] == (p_lit ^ 1) {
                        //         potential_uip = p_lit ^ 1;
                        //         lit_count += 1;
                        //     }
                        // }
                    }

                    // If there is only one literal from the current decision level left in the clause..
                    // backtrack and learn clause.
                    if lit_count == 1 {
                        uip = potential_uip;
                        println!("FOUND UIP: {}", self.lit_to_string(uip));
                        break;
                    }              
                }
                
                // let mut found_uip = false;
                // for x in 0..self.m_decision_level_implications[self.m_decision_level as usize].len() {
                //     let last_impl = &self.m_decision_level_implications
                //                     [self.m_decision_level as usize]
                //                     [self.m_decision_level_implications[self.m_decision_level as usize].len() - x - 1];
                    





                //     if last_impl.uniq_implication_point == true {
                //         uip = last_impl.literal;
                //         found_uip = true;
                //     }
                // }

                // If no UIP was found within the propagated units, go with decision variable itself.

                // ASGASGFASGF
                // if !found_uip {
                //     uip = self.m_decision_guesses.keys().last().unwrap().clone();
                // }

                // Now find all things implied by the UIP, and construct conflict clause.
                // for x in 0..self.m_decision_level_implications[self.m_decision_level as usize].len() {
                //     let last_impl = &self.m_decision_level_implications
                //                     [self.m_decision_level as usize]
                //                     [self.m_decision_level_implications[self.m_decision_level as usize].len() - x - 1];
                    
                //     if last_impl.implied_by_vars.contains(&uip) {
                //         for var in &last_impl.implied_by_vars {
                //             println!("Debug: var({}) uip({}) {:?}", var, uip, self.lit_list_to_strings(last_impl.implied_by_vars.clone()));
                //             if !conflict_clause.contains(&(var ^ 1)) {
                //                 conflict_clause.push(var ^ 1);
                //             }
                //         }
                //     }
                // }
                
                println!("Conflict Clause: {:?}", self.lit_list_to_strings(conflict_clause.clone()));

                // FIND BACKTRACK LEVEL (Highest decision level guess in new learned clause)

                // for literal in conflict_clause.clone() {
                //     if literal != uip && self.m_assignments.contains_key(&(literal^1)) {
                //         bt_level = cmp::max(self.m_assignments.get(&(literal^1)).unwrap().clone(), bt_level);
                //     }
                // }


                // for literal in &conflict_clause {
                //     if self.m_decision_guesses.contains_key(&(literal^1)) {
                //         let literal_decision_level = self.m_decision_guesses.get(&(literal^1)).unwrap();
                //         if literal_decision_level != &self.m_decision_level {
                //             if literal_decision_level > &bt_level {
                //                 bt_level = *literal_decision_level;
                //             }
                //         }
                //     }
                // }

                for literal in &conflict_clause {
                    if self.m_assignments.contains_key(&(literal^1)) {
                        let potential_bt = self.m_assignments.get(&(literal^1)).unwrap().clone();
                        if potential_bt > bt_level && potential_bt < self.m_decision_level {
                            bt_level = potential_bt;
                        }
                    }
                }

                println!("bt_level {}", bt_level);


                // for guess in self.m_decision_guesses.iter() {
                //     if partial_learned_clause.contains(&(guess.0 ^ 1)) {
                //         bt_level = guess.1.clone();
                //     }
                // }

                break;
            }

            if currently_sat {
                    sat_count += 1;
            }
        }

        // println!("Propagate Count: {}", propagate_count);
        // if propagate_count == 1 {
        //     self.m_decision_level_implications[self.m_decision_level as usize].last_mut().unwrap().uniq_implication_point = true;
        // }

        //println!("sat count: {}, clauses len: {}", sat_count, self.m_clauses.len());



        if conflict {
            if bt_level == -1 {
                println!("UNSAT");
                self.m_finished = true;
                return true;
            }
            // Push conflict clause.
            self.m_clauses.push(conflict_clause);

            //println!("New Clauses: {:?}", self.m_clauses);

            // for _x in 0..(self.m_decision_level - bt_level) {
            //     // Pop off vectors from current decision level.
            //     //self.m_saved_assignments.pop();
            //     self.m_decision_guesses.pop();
            //     self.m_decision_level_implications.pop();
            // }

            

            for (key, value) in &self.m_assignments.clone() {
                if value >= &bt_level {
                    self.m_assignments.remove(key);
                }
            }
            for (key, value) in &self.m_decision_guesses.clone() {
                if value > &bt_level {
                    self.m_decision_guesses.remove(key);
                }
            }

            // Set decision level to backtrack level.
            self.m_decision_level = bt_level;
   
            // Pop off and insert flipped in each.
            let flipped = self.m_decision_guesses.iter().last().unwrap().0.clone() ^ 1;
            self.m_decision_guesses.pop();
            self.m_assignments.insert(flipped, self.m_decision_level);
            self.m_decision_guesses.insert(flipped, self.m_decision_level);

            // Insert UIP (at higher decision level)
            self.m_assignments.insert(uip, self.m_decision_level);
        }


        else if sat_count == self.m_clauses.len() {
            println!("Solution:\n{:?}", self.m_assignments);
            self.m_finished = true;
        }

        println!("m_runs: {}", self.m_runs);
        self.m_runs += 1;


        return true;
    }
}