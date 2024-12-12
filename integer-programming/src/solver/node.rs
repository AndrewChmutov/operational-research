use std::marker::PhantomData;

use crate::problem::{Bounds, ProblemIR};
use crate::utils::is_float;

use super::variable::VariableSearch;

fn is_milp_solution(values: &[f64], integer: &[bool]) -> bool {
    values
        .iter()
        .zip(integer.iter())
        .all(|(val, int)| !int || *int && !is_float(*val))
}

pub trait NodeSearch {
    fn solve<V: VariableSearch>(problem: &ProblemIR, bounds: Bounds) -> (f64, u32);
}

pub trait NodeSort {
    fn sort(bounds: &mut [Option<Bounds>]);
}

pub struct NoSort;
impl NodeSort for NoSort {
    fn sort(_bounds: &mut [Option<Bounds>]) {}
}

pub struct ByInterval;
impl NodeSort for ByInterval {
    fn sort(bounds: &mut [Option<Bounds>]) {
        bounds.sort_by(|a, b| {
            b.as_ref()
                .map_or(0.0, |x| x.total_length())
                .total_cmp(&a.as_ref().map_or(0.0, |x| x.total_length()))
        });
    }
}

pub struct DFS<T: NodeSort> {
    _marker: PhantomData<T>,
}

impl<T: NodeSort> DFS<T> {
    fn solve_rec<V>(problem: &ProblemIR, bounds: Bounds, lower: &mut f64, solver_calls: &mut u32)
    where
        V: VariableSearch,
    {
        *solver_calls += 1;
        // Solve relaxed problem
        let (solution, values) = match problem.with_bounds(&bounds) {
            // Unfeasible
            None => return,

            // Feasible
            Some((solution, values)) => (solution, values),
        };
        println!("Current solution: {}", solution);

        // Maximization problem
        // And relaxed problem is already worse,
        // than encountered non-relaxed one
        if solution <= *lower {
            return;
        }
        // Check whether solution is of MILP
        // if so => update the lower bound
        else if is_milp_solution(&values, &problem.is_integer) {
            *lower = solution;
            return;
        }

        // Integer variables with real values
        let integer_but_real = V::pick(problem, &values, &bounds);

        if let Some(i) = integer_but_real {
            let (left_bounds, right_bounds) = bounds.split(i, values[i]);
            let mut bounds = [left_bounds, right_bounds];
            T::sort(&mut bounds);
            for bound in bounds.into_iter().flatten() {
                Self::solve_rec::<V>(problem, bound, lower, solver_calls);
            }
        }
    }
}

impl<T: NodeSort> NodeSearch for DFS<T> {
    fn solve<V: VariableSearch>(problem: &ProblemIR, bounds: Bounds) -> (f64, u32) {
        // Initialize stack
        let mut solver_calls = 0u32;
        let mut lower = 0.0f64;

        // Solve recursively
        Self::solve_rec::<V>(problem, bounds, &mut lower, &mut solver_calls);
        (lower, solver_calls)
    }
}
