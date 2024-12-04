use crate::utils::is_zero;

use good_lp::{
    constraint, default_solver, variable, ProblemVariables, Solution, SolverModel, Variable,
    VariableDefinition,
};

pub struct ProblemIR {
    pub coefficients: Vec<Vec<f64>>,
    pub objective_coefficients: Vec<f64>,
    pub resources: Vec<f64>,
    pub is_integer: Vec<bool>,
}

#[derive(Clone, Debug)]
pub struct Bounds {
    pub lb: Vec<f64>,
    pub ub: Vec<f64>,
}

impl Bounds {
    pub fn split(&self, i: usize, value: f64) -> (Option<Self>, Option<Self>) {
        if self.lb[i] == self.ub[i] {
            return (None, None);
        }
        // Update upper bounds
        debug_assert!(!is_zero(value));
        let new_left_ub = value.floor();
        let new_right_lb = value.ceil();
        dbg!(&new_left_ub);
        dbg!(&new_right_lb);

        let left = if self.lb[i] <= new_left_ub {
            let mut bounds = self.clone();
            bounds.ub[i] = new_left_ub;
            Some(bounds)
        } else {
            None
        };
        let right = if self.ub[i] >= new_right_lb {
            let mut bounds = self.clone();
            bounds.lb[i] = new_right_lb;
            Some(bounds)
        } else {
            None
        };

        dbg!((left, right))
    }

    pub fn variables(&self) -> impl Iterator<Item = VariableDefinition> + '_ {
        self.lb
            .iter()
            .zip(self.ub.iter())
            .map(|(lb, ub)| variable().min(*lb).max(*ub))
    }
}

impl ProblemIR {
    pub fn with_bounds(&self, bounds: &Bounds) -> Option<(f64, Vec<f64>)> {
        // Define variables
        let mut problem_variables = ProblemVariables::new();
        let variables = bounds
            .variables()
            // Collect variables
            .map(|x| problem_variables.add(x))
            .collect::<Vec<Variable>>();

        // Create objective as a sum over variables
        let objective = variables
            .iter()
            .zip(self.objective_coefficients.iter())
            .skip(1)
            .fold(self.objective_coefficients[0] * variables[0], |sum, x| {
                sum + *x.0 * (*x.1)
            });

        // Create problem with defined objective
        let mut model = problem_variables.maximise(objective).using(default_solver);

        // Disable model output
        model.set_parameter("loglevel", "0");

        // Add constraints
        for constr in self
            .coefficients
            .iter()
            // Create expressions
            .map(|x| {
                x.iter()
                    .zip(variables.iter())
                    .skip(1)
                    .fold(x[0] * variables[0], |sum, x| sum + *x.0 * (*x.1))
            })
            // Create constraints
            .zip(self.resources.iter())
            .map(|(expression, resource)| constraint!(expression <= *resource))
        {
            dbg!(&constr);
            model = model.with(constr);
        }

        model.solve().ok().map(|x| {
            let objective_value = x.model().obj_value();
            println!("Variables {:?}", variables);
            println!("Bounds {:?}", variables);
            let variable_values = variables
                .into_iter()
                .map(|v| x.value(v))
                .collect::<Vec<_>>();
            println!("Values {:?}", variable_values);
            (objective_value, variable_values)
        })
    }
}
