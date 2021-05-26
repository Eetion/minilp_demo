
use minilp::{Problem, OptimizationDirection, ComparisonOp};


fn main() {

    // PROBLEM 1 
    // --------------------------------------------------------------------------------

    // Maximize an objective function x + 2 * y of two variables x >= 0 and 0 <= y <= 3
    let mut problem = Problem::new(OptimizationDirection::Maximize);
    let x = problem.add_var(1.0, (0.0, f64::INFINITY));
    let y = problem.add_var(2.0, (0.0, 3.0));

    // subject to constraints: x + y <= 4 and 2 * x + y >= 2.
    problem.add_constraint(&[(x, 1.0), (y, 1.0)], ComparisonOp::Le, 4.0);
    //problem.add_constraint(&[(x, 2.0), (y, 1.0)], ComparisonOp::Ge, 2.0);
    problem.add_constraint(&[(x, 2.0), ], ComparisonOp::Ge, 2.0);

    // Optimal value is 7, achieved at x = 1 and y = 3.
    let solution = problem.solve().unwrap();
    assert_eq!(solution.objective(), 7.0);
    assert_eq!(solution[x], 1.0);
    assert_eq!(solution[y], 3.0);


    // PROBLEM 2 
    // --------------------------------------------------------------------------------
    
    let mut problem2 = Problem::new(OptimizationDirection::Maximize);
    let mut variables = Vec::new();


    let n_var = 4;

    for _i in 0..n_var {
        let t = problem2.add_var(1.0, (0.0, f64::INFINITY));
        variables.push(t);
    }

    for i in 0..n_var {
        problem2.add_constraint( &[(variables[i], 1.0), ], ComparisonOp::Le, 1.0)
    }

    // Get solution
    let solution2 = problem2.solve().unwrap();

    // Print the variables stored in our vector
    println!("Problem variables:");
    println!("{:?}", variables.clone());

    // Print solution
    println!("Problem solution:");
    let vector : Vec<_> = solution2.iter().map(|x| x.clone()).collect();
    println!("{:?}", vector);

    
}

