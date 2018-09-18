#![allow(dead_code)]


use indexmap::IndexMap;
use rand::prelude::*;

pub struct ImplicationInformation {
    pub literal: u32,
    pub implied_by_vars: Vec<u32>,
    pub implied_by_clause: Vec<u32>,
}

pub struct CNFFormula {
    pub m_decide_count: u32,
    pub m_finished: bool,
    pub m_clauses: Vec<Vec<u32>>,
    pub m_variables: IndexMap<String, u32>,
    pub m_assignments: IndexMap<u32, i32>,
    pub m_decisions: Vec<u32>,
    pub m_decision_level: i32,
    pub m_implications: Vec<Vec<ImplicationInformation>>,
}

impl CNFFormula {
    pub fn add_clause(variables: &IndexMap<String, u32>, clause: Vec<String>) -> Vec<u32> {
        let mut literals: Vec<u32> = Vec::new();

        for literal in clause {
            if literal.starts_with("-") {
                literals.push(variables.get(&literal.trim_matches('-').to_owned()).unwrap().clone() << 1 | 1)
            }
            else if !literal.starts_with("0") {
                literals.push(variables.get(&literal).unwrap().clone() << 1)
            }
        }        
        
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
 
        return CNFFormula{ m_decide_count: 1, m_finished: false,
                           m_clauses: clauses , m_variables: variables,
                           m_decisions: Vec::new(),
                           m_assignments: IndexMap::new(), 
                           m_decision_level: 0,
                           m_implications: Vec::new() };
    } 

    pub fn assignment_find_decision_level(&self, literal: &u32) -> Option<i32> {
        if self.m_assignments.contains_key(literal) {
            return Some(self.m_assignments.get(literal).unwrap().clone());
        }
        else if self.m_assignments.contains_key(&(literal ^ 1)) {
            return Some(self.m_assignments.get(&(literal ^ 1)).unwrap().clone());
        }
        else {
            return None
        }
    }

    pub fn assignments_find_at_decisionlevel(&self, literal: &u32, dec_level: &i32) -> bool {
        for (key, value) in &self.m_assignments {
            if value == dec_level {
                if key == &(literal ^ 1) {
                    return true;
                }
                else if key == literal {
                    return true;
                }
            }
        }
        return false;
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
        for imp_info in &self.m_implications[self.m_decision_level as usize] {
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
        if self.m_decision_level == 0 {
            self.m_implications.push(Vec::new());
        }

        self.m_decision_level += 1;
        self.m_decide_count += 1;

        self.m_implications.push(Vec::new());

        let mut rng = thread_rng();

        let mut index = rng.gen_range(0, self.m_variables.len());
        let zero_or_one = rng.gen_range(0, 2);

        while self.m_assignments.contains_key(&((self.m_variables.get_index(index).unwrap().1.clone() << 1) | 1)) ||
              self.m_assignments.contains_key(&(self.m_variables.get_index(index).unwrap().1.clone() << 1)) { //&&
              //self.m_assignments.len() != self.m_variables.len() {
            index = rng.gen_range(0, self.m_variables.len());
        }

        self.m_assignments.insert(*self.m_variables.get_index(index).unwrap().1 << 1 | zero_or_one, self.m_decision_level);
        self.m_decisions.push(*self.m_variables.get_index(index).unwrap().1 << 1 | zero_or_one);

        // let implication_info = ImplicationInformation {
        //     literal: *self.m_variables.get_index(index).unwrap().1 << 1,
        //     implied_by_vars: Vec::new(),
        //     implied_by_clause: Vec::new()
        // };

        // self.m_implications[self.m_decision_level as usize].push(implication_info);
    }

    pub fn make_decision_fake(&mut self, decision: u32, truthval: bool) {
        if self.m_decision_level == 0 {
            self.m_implications.push(Vec::new());
        }

        self.m_decision_level += 1;
        self.m_implications.push(Vec::new());

        let polarity;

        if truthval {
            polarity = 0;
        } else {
            polarity = 1;
        }

        self.m_assignments.insert(*self.m_variables.get_index(decision as usize).unwrap().1 << 1 | polarity, self.m_decision_level);
        self.m_decisions.push(*self.m_variables.get_index(decision as usize).unwrap().1 << 1 | polarity);
    }

    pub fn print_stats(&self) {
        println!("Current clause count: {}", self.m_clauses.len());
        println!("Current assignment count: {}", self.m_assignments.len());
        println!("Current decision level: {}", self.m_decision_level);
    }

    pub fn restart(&mut self) {
        self.m_implications.clear();
        self.m_decision_level = 0;
        self.m_assignments.clear();
        self.m_decisions.clear();
        self.m_decide_count = 1;
    }

    pub fn solve(&mut self) -> bool {
        println!();
        self.print_assignments(&self.m_assignments);

        // For potential conflict
        let mut conflict = false;
        let mut conflict_clause: Vec<u32> = Vec::new();

        // Assertion level and counter for satisfied clauses.
        let mut a_level: i32 = -1;
        let mut sat_count = 0;

        // p variable
        let mut p_true = false;
        let mut p = u32::max_value();

        // Process clauses.
        for clause in &self.m_clauses {
            let mut free_literals: Vec<u32> = Vec::new();
            let mut implication_literals: Vec<u32> = Vec::new();
            let mut currently_sat = false;

            for &literal in clause {
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

                println!("Propagating variable {}", self.lit_to_string(implication_info.literal.clone()));
                self.print_implications(&implication_info);

                self.m_implications[self.m_decision_level as usize].push(implication_info);
                self.m_assignments.insert(free_literals[0], self.m_decision_level);

                // Return so we propagate all possible units.
                return false;
            }

            //If there is a conflict, add the clause
            if free_literals.len() == 0 && !currently_sat {
                println!("Found conflict: {:?}", self.lit_list_to_strings(clause.clone()));

                conflict = true;

                if self.m_decision_level == 0 {
                    break;
                }

                // Find learned clause, and UIP (Unique Implication Point) in the process.
                conflict_clause = clause.clone();

                //p = self.m_implications[self.m_decision_level as usize][0].literal ^ 1;
                while let Some(last_impl) = self.m_implications[self.m_decision_level as usize].pop() {
                //for last_impl in self.m_implications[self.m_decision_level as usize].iter().rev() {

                    self.print_implications(&last_impl);

                    conflict_clause = Self::update_partial_clause(&conflict_clause, &last_impl.implied_by_clause);


                    // Count literals at current decision level that are in clause.
                    let mut lit_count = 0;
                    for p_lit in &conflict_clause {
                        if self.assignments_find_at_decisionlevel(p_lit, &self.m_decision_level) {
                            p = p_lit.clone();
                            lit_count += 1;
                        }
                        // for (key, value) in &self.m_assignments {
                        //     if value == &self.m_decision_level {
                        //         if key == &(p_lit ^ 1) {
                        //             lit_count += 1;
                        //         }
                        //         else if key == p_lit {
                        //             lit_count += 1;
                        //         }
                        //     }
                        // }
                    }

                    // If there is only one literal from the current decision level left in the clause we found UIP,
                    // backtrack to assertion level and learn clause.
                    if lit_count == 1 {
                        p_true = true;
                        break;
                    }
                }

                println!("______________________________________________________________________");
                self.print_current_level_implications();
                println!("______________________________________________________________________");
                
                println!("p is {}", self.lit_to_string(p.clone()));

    
                if !p_true {
                    println!("getting last decision:");
                    p = self.m_decisions.last().unwrap().clone();
                    let mut new_conf_clause = Vec::new();
                    for i in 0..conflict_clause.len() {
                        let dec_level = self.assignment_find_decision_level(&conflict_clause[i]);
                        match dec_level {
                            Some(x) => {
                                if x == self.m_decision_level {
                                    if conflict_clause[i] != p ^ 1 {
                                        continue;
                                    }
                                }
                                new_conf_clause.push(conflict_clause[i]);
                            },
                            None => {}
                        }
                    }
                    conflict_clause = new_conf_clause;
                }

                println!("Literal assigned at conflict level: {}", p_true);


                println!("Learned clause: {:?}", self.lit_list_to_strings(conflict_clause.clone()));

                // Here, if conflict clause is one and the literal is of same decision level as current,
                // set assertion level to 0 by convention.
                if conflict_clause.len() == 1 {
                    let lit_dec_level = self.assignment_find_decision_level(&conflict_clause[0]);
                    match lit_dec_level {
                        Some(x) => { 
                            if x == 0 { 
                                a_level = -1; 
                            } 
                            else { 
                                a_level = 0; 
                            }
                        },
                        None => {}
                    }
                    // if self.m_assignments.contains_key(&conflict_clause[0]) {
                    //     let lit_dec_level = self.m_assignments.get(&conflict_clause[0]).unwrap().clone();
                    //     if lit_dec_level == 0 {
                    //         a_level = -1;
                    //     }
                    //     else {
                    //         a_level = 0;
                    //     }
                    // }
                    // else if self.m_assignments.contains_key(&(conflict_clause[0] ^ 1)) {
                    //     let lit_dec_level = self.m_assignments.get(&(conflict_clause[0] ^ 1)).unwrap().clone();
                    //     if lit_dec_level == 0 {
                    //         a_level = -1;
                    //     }
                    //     else {
                    //         a_level = 0;
                    //     }
                    // }
                }
                else {
                    let mut max = -1;
                    let mut second_max = -1;

                    for literal in &conflict_clause {
                        let lit_dec_level = self.assignment_find_decision_level(&literal);
                        println!("LDL {:?}", lit_dec_level);
                        match lit_dec_level {
                            Some(x) => {
                                if x > second_max {
                                    if x > max {
                                        second_max = max;
                                        max = x;
                                    }
                                    else if x != max {
                                        second_max = x;
                                    }
                                }
                                // if x > max { 
                                //     second_max = max; 
                                //     max = x;
                                // } else if x > second_max {//if x != max && { whatcode
                                //     second_max = x;
                                // } 
                            },
                            None => {}
                        }
                        // if self.m_assignments.contains_key(literal) {
                        //     let lit_dec_level = self.m_assignments.get(literal).unwrap().clone();
                        //     if lit_dec_level > second_max {
                        //         if lit_dec_level > max {
                        //             second_max = max;
                        //             max = lit_dec_level
                        //         } 
                        //         else if lit_dec_level != max {
                        //             second_max = lit_dec_level;
                        //         }           
                        //     }
                        // }
                        // else if self.m_assignments.contains_key(&(literal ^ 1)) {
                        //     let lit_dec_level = self.m_assignments.get(&(literal ^ 1)).unwrap().clone();
                        //     if lit_dec_level > second_max {
                        //         if lit_dec_level > max {
                        //             second_max = max;
                        //             max = lit_dec_level
                        //         } 
                        //         else if lit_dec_level != max {
                        //             second_max = lit_dec_level;
                        //         }           
                        //     }
                        // }
                    }

                    // If there is only one decision level in clause.
                    if second_max == -1 {
                        a_level = max;
                    } 
                    // Else a_level is the second highest level of a literal in the learned clause.
                    else {
                        a_level = second_max;
                    }

                    // If the assertion level is current level, -1.
                    if a_level == self.m_decision_level {
                        a_level = self.m_decision_level - 1;
                    }


                }
                println!("Assertion Level: {}", a_level);
                break;
            }
            
            if currently_sat { sat_count += 1; }
        }

        if conflict {
            if a_level == -1 {
                println!("UNSAT");
                self.m_finished = true;
                return true;
            }

            // Push conflict clause.
            self.m_clauses.push(conflict_clause);
      
            // Remove assignments that exceed backtrack value.
            for (key, value) in &self.m_assignments.clone() {
                if value > &a_level {
                    self.m_assignments.remove(key);            
                }
            }

            println!("Removing from {:?} @ dec_level {}", self.m_decisions, self.m_decision_level);
            for _i in 0..self.m_decision_level - a_level {
                self.m_decisions.pop();
            }
            println!("Result is {:?}", self.m_decisions);


            self.m_decision_level = a_level;

            // Return so we can do solving...
            return false;
        }
        
        if sat_count == self.m_clauses.len() {
            println!("Solution:\n{:?}", self.m_assignments);
            self.m_finished = true;
        }

        return true;
    }
}