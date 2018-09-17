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
            //////println!("Clause: {:?}", clause);
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


        //////println!("Variables: {:?}", variables);


 
        return CNFFormula{ m_decide_count: 0, m_finished: false,
                           m_clauses: clauses , m_variables: variables,
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
        self.m_decide_count += 1;

        self.m_decision_level_implications.push(Vec::new());

        let mut rng = thread_rng();

        let mut index = rng.gen_range(0, self.m_variables.len());
        let negated = rng.gen_range(0, 2);

        while self.m_assignments.contains_key(&((self.m_variables.get_index(index).unwrap().1.clone() << 1) | 1)) ||
              self.m_assignments.contains_key(&(self.m_variables.get_index(index).unwrap().1.clone() << 1)) &&
              self.m_assignments.len() != self.m_variables.len() {
                  //println!("assign_len {}, var_len {}", self.m_assignments.len(), self.m_variables.len());
            index = rng.gen_range(0, self.m_variables.len());
        }

        self.m_decision_guesses.insert(*self.m_variables.get_index(index).unwrap().1 << 1 | negated, self.m_decision_level);
        self.m_assignments.insert(*self.m_variables.get_index(index).unwrap().1 << 1 | negated, self.m_decision_level);
        //println!("Made decision {}", *self.m_variables.get_index(index).unwrap().1 << 1 | negated);  
    }

    pub fn make_decision_fake(&mut self, decision: u32, truthval: bool) {
        self.m_decision_level += 1;
        self.m_decision_level_implications.push(Vec::new());

        if truthval {
            self.m_decision_guesses.insert(*self.m_variables.get_index(decision as usize).unwrap().1 << 1 | 0, self.m_decision_level);
            self.m_assignments.insert(*self.m_variables.get_index(decision as usize).unwrap().1 << 1 | 0, self.m_decision_level);
            //println!("Made 'fake' decision {}", *self.m_variables.get_index(decision as usize).unwrap().1 << 1 | 0);
        } else {
            self.m_decision_guesses.insert(*self.m_variables.get_index(decision as usize).unwrap().1 << 1 | 1, self.m_decision_level);
            self.m_assignments.insert(*self.m_variables.get_index(decision as usize).unwrap().1 << 1 | 1, self.m_decision_level);
            //println!("Made 'fake' decision {}", *self.m_variables.get_index(decision as usize).unwrap().1 << 1 | 1);
        }
    }

    pub fn restart(&mut self) {
        self.m_decision_guesses.clear();
        self.m_decision_level_implications.clear();
        self.m_decision_level = -1;
        self.m_assignments.clear();
        self.m_decide_count = 0;
    }

    pub fn solve(&mut self) -> bool {
        //println!("CURRENT ASSIGNMENTS ----- {:?}", self.m_decision_guesses);
        //println!("CURRENT ASSIGNMENTS ----- {:?}", self.m_assignments);
        //////self.print_assignments(&self.m_assignments);

        //let mut literal_assignments: Vec<u32> = Vec::new();

        // For potential conflict
        let mut conflict = false;
        let mut conflict_clause: Vec<u32> = Vec::new();
        //let mut partial_learned_clause: Vec<u32> = Vec::new();
        let mut bt_level: i32 = -1;

        // UIP var
        let mut sat_count = 0;
        let mut uip = 0;

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

                //////self.print_implications(&implication_info);
                self.m_decision_level_implications[self.m_decision_level as usize].push(implication_info);
                self.m_assignments.insert(free_literals[0], self.m_decision_level);

                //return false;
            }

            //If there is a conflict, add the clause
            if free_literals.len() == 0 && !currently_sat {
                conflict = true;

                // Start the partial learned clause by cloning the current clause.
                //////println!("Conflict! {:?} @ decision_level = {}", self.lit_list_to_strings(clause.clone()), self.m_decision_level);

                // FIND CLAUSE TO LEARN.
                // Find UIP
                conflict_clause = clause.clone();
                while let Some(last_impl) = self.m_decision_level_implications[self.m_decision_level as usize].pop() {
                //for last_impl in self.m_decision_level_implications[self.m_decision_level as usize].iter().rev() {

                    ////self.print_implications(&last_impl);

                    conflict_clause = Self::update_partial_clause(&conflict_clause, &last_impl.implied_by_clause);

                    // Count literals at current decision level that are in clause. (Also find UIP).
                    let mut lit_count = 0;
                    let mut potential_uip = 0;
                    for p_lit in &conflict_clause {
                        for (key, value) in self.m_assignments.iter() {
                            if value == &self.m_decision_level {
                                if key == &(p_lit^1) {
                                    potential_uip = p_lit ^ 1;
                                    lit_count += 1;
                                }
                                else if key == p_lit {
                                    potential_uip = p_lit.clone();
                                    lit_count += 1;
                                }
                            }
                        }
                    }

                    // If there is only one literal from the current decision level left in the clause..
                    // backtrack and learn clause.
                    if lit_count == 1 {
                        uip = potential_uip;
                        //println!("FOUND UIP: {}", self.lit_to_string(uip));
                        break;
                    }              
                }

                for literal in &conflict_clause {
                    if literal != &uip && literal != &(uip ^ 1) {
                    //if literal != &uip && literal != &(uip ^ 1) && self.m_assignments.contains_key(&(literal^1)) { //|| self.m_assignments.contains_key(literal)) {             
                        if self.m_assignments.contains_key(&(literal^1)) {
                            let potential_bt = self.m_assignments.get(&(literal^1)).unwrap().clone();
                            if potential_bt > bt_level { //ISSUE HERE
                                bt_level = potential_bt;
                            }
                        } 
                        else if self.m_assignments.contains_key(literal) {
                            let potential_bt = self.m_assignments.get(literal).unwrap().clone();
                            if potential_bt > bt_level { //ISSUE HERE
                                bt_level = potential_bt;
                            }
                        }
                    }
                }

                // Here, if conflict clause is one and the literal is of same decision level as current,
                // set to 0 by convention.
                if conflict_clause.len() == 1 {
                        if self.m_assignments.contains_key(&conflict_clause[0]) {
                            let pot_bt = self.m_assignments.get(&conflict_clause[0]).unwrap().clone();
                            if pot_bt == self.m_decision_level {
                                bt_level = 0;
                            }
                            else {
                                bt_level = -1;
                            }
                        }
                        else if self.m_assignments.contains_key(&(conflict_clause[0] ^ 1)) {
                            let pot_bt = self.m_assignments.get(&(conflict_clause[0] ^ 1)).unwrap().clone();
                            if pot_bt == self.m_decision_level {
                                bt_level = 0;
                            } 
                            else {
                                bt_level = -1;
                            }
                        }  
                }

                break;
            }
            
            if currently_sat { sat_count += 1; }
        }

        if conflict {
            if bt_level == -1 {
                println!("UNSAT");
                self.m_finished = true;
                return true;
            }
            // Push conflict clause.
            self.m_clauses.push(conflict_clause);
      
            // Remove assignments that exceed backtrack value.
            for (key, value) in &self.m_assignments.clone() {
                if value >= &bt_level {
                    self.m_assignments.remove(key);
                }
            }

            // Remove assignments that exceed backtrack value.
            for (key, value) in &self.m_decision_guesses.clone() {
                if value >= &bt_level {
                    self.m_decision_guesses.remove(key);
                }
            }

            // Insert UIP (at higher decision level)
            //if push_uip { self.m_assignments.insert(uip, self.m_decision_level); } 

            // Set decision level to backtrack level. 
            self.m_decision_level = bt_level;
            // Skip an increment, this way we set the decision at the current level.
            // self.m_increment_on_decision = false;

            //return false;
   
            // Pop off and insert flipped in each.
            // let flipped = self.m_decision_guesses.iter().last().unwrap().0.clone() ^ 1;
            // self.m_decision_guesses.pop();
            // self.m_assignments.insert(flipped, self.m_decision_level);
            // self.m_decision_guesses.insert(flipped, self.m_decision_level);    
        }
        
        if sat_count == self.m_clauses.len() {
            println!("Solution:\n{:?}", self.m_assignments);
            self.m_finished = true;
        }
        return true;
    }
}