extern crate datafrog;

//use std::cell::RefCell;

//use datafrog::Iteration;


fn to_parent_id  (child_id: u32) -> u32        { child_id / 3 }
fn to_parent_fact(child_id: u32) -> (u32, u32) { (to_parent_id(child_id), child_id) }

fn apply_rule_sibling(parent_id: &u32, first_child: &u32, second_child: &u32) -> (u32, u32) {
    println!("Matched parent: {:?} for children: {:?}, {:?}", parent_id, first_child, second_child);

    if *first_child == *second_child { (*first_child, *second_child) }
    else                             { (*first_child, *second_child) }
}

fn main() {
    // ------------------------------
    // Preparation

    // Start with some facts
    //let parent_facts = RefCell::new();
    let parent_facts = datafrog::Relation::from((0..9).map(to_parent_fact));

    println!("Parent Facts:\t{:?}", parent_facts.elements);

    let timer = ::std::time::Instant::now();

    // ------------------------------
    // Datalog logic

    // Create the datalog iteration context
    let mut datafrog_iteration = datafrog::Iteration::new();

    // variables in an iteration are akin to "dynamic relations" rather than "static relations"
    let parent_predicate   = datafrog_iteration.variable::<(u32,u32)>("parent"  );
    let sibling_predicate  = datafrog_iteration.variable::<(u32,u32)>("sibling" );
    //let relative_predicate = datafrog_iteration.variable::<(u32,u32)>("relative");

    // load predicate with initial values
    parent_predicate.insert(datafrog::Relation::from(parent_facts.elements));

    // iterate the rules!
    while datafrog_iteration.changed() {
        // sibling(First_child, Second_child) <- parent(First_child, Parent), parent(Second_child, Parent)

        sibling_predicate.from_join(
            &parent_predicate,
            &parent_predicate,
            apply_rule_sibling
            //| _parent_id, &first_child, &second_child | (first_child, second_child)
        );


        //println!("Parent Facts:\t{:?}", parent_facts.elements);
        println!("stable sibling count: {:?}", sibling_predicate.stable.as_ref().borrow().len());
        println!("recent siblings: {:?}", sibling_predicate.recent.as_ref().borrow().elements);

        /*
        relative_predicate.insert(
        relative_predicate.from_join(
            &parent_predicate,
            &sibling_predicate,
            |_b, &a, &c| (c,a)
        );
        */
    }

    let all_siblings = sibling_predicate.complete();

    println!("{:?}\tComputation complete (sibling count: {:?})", timer.elapsed(), all_siblings.len());
    println!("Siblings relation: {:?}", all_siblings.elements);
}
