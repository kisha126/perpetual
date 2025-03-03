use extendr_api::prelude::*;
use perpetual_rs::booster::booster::MissingNodeTreatment;
use perpetual_rs::booster::multi_output::MultiOutputBooster as CrateMultiOutputBooster;
use perpetual_rs::constraints::Constraint;
use perpetual_rs::data::Matrix;
use perpetual_rs::objective::Objective;
use std::collections::{HashMap, HashSet};
use crate::utils::int_map_to_constraint_map;

#[extendr]
pub struct MultiOutputBooster {
    booster: CrateMultiOutputBooster,
}

#[extendr]
impl MultiOutputBooster {
    fn new(
        n_boosters: usize,
        objective: &str,
        budget: f32,
        max_bin: u16,
        num_threads: Option<usize>,
        monotone_constraints: HashMap<usize, i8>,
        force_children_to_bound_parent: bool,
        missing: f64,
        allow_missing_splits: bool,
        create_missing_branch: bool,
        terminate_missing_features: HashSet<usize>,
        missing_node_treatment: &str,
        log_iterations: usize,
        quantile: Option<f64>,
        reset: Option<bool>,
        categorical_features: Option<HashSet<usize>>,
        timeout: Option<f32>,
        iteration_limit: Option<usize>,
        memory_limit: Option<f32>,
        stopping_rounds: Option<usize>,
    ) -> Result<Robj> {
        let objective_ = serde_plain::from_str(objective).map_err(|e| e.to_string())?;
        let missing_node_treatment_ = serde_plain::from_str(missing_node_treatment).map_err(|e| e.to_string())?;
        let monotone_constraints_ = int_map_to_constraint_map(monotone_constraints).map_err(|e| e.to_string())?;

        let booster = CrateMultiOutputBooster::default()
            .set_objective(objective_)
            .set_budget(budget)
            .set_max_bin(max_bin)
            .set_num_threads(num_threads)
            .set_monotone_constraints(Some(monotone_constraints_))
            .set_force_children_to_bound_parent(force_children_to_bound_parent)
            .set_missing(missing)
            .set_allow_missing_splits(allow_missing_splits)
            .set_create_missing_branch(create_missing_branch)
            .set_terminate_missing_features(terminate_missing_features)
            .set_missing_node_treatment(missing_node_treatment_)
            .set_log_iterations(log_iterations)
            .set_n_boosters(n_boosters)
            .set_quantile(quantile)
            .set_reset(reset)
            .set_categorical_features(categorical_features)
            .set_timeout(timeout)
            .set_iteration_limit(iteration_limit)
            .set_memory_limit(memory_limit)
            .set_stopping_rounds(stopping_rounds);

        Ok(MultiOutputBooster { booster })
    }

    fn set_n_boosters(&mut self, value: usize) -> Result<(), String> {
        self.booster = self.booster.clone().set_n_boosters(value);
        Ok(())
    }

    fn set_objective(&mut self, value: &str) -> Result<(), String> {
        let objective_ = serde_plain::from_str(value).map_err(|e| e.to_string())?;
        self.booster = self.booster.clone().set_objective(objective_);
        Ok(())
    }

    fn set_budget(&mut self, value: f32) -> Result<(), String> {
        self.booster = self.booster.clone().set_budget(value);
        Ok(())
    }

    fn set_max_bin(&mut self, value: u16) -> Result<(), String> {
        self.booster = self.booster.clone().set_max_bin(value);
        Ok(())
    }

    fn set_num_threads(&mut self, value: Option<usize>) -> Result<(), String> {
        self.booster = self.booster.clone().set_num_threads(value);
        Ok(())
    }

    fn set_monotone_constraints(&mut self, value: HashMap<usize, i8>) -> Result<(), String> {
        let map = int_map_to_constraint_map(value).map_err(|e| e.to_string())?;
        self.booster = self.booster.clone().set_monotone_constraints(Some(map));
        Ok(())
    }

    fn set_force_children_to_bound_parent(&mut self, value: bool) -> Result<(), String> {
        self.booster = self.booster.clone().set_force_children_to_bound_parent(value);
        Ok(())
    }

    fn set_missing(&mut self, value: f64) -> Result<(), String> {
        self.booster = self.booster.clone().set_missing(value);
        Ok(())
    }

    fn set_allow_missing_splits(&mut self, value: bool) -> Result<(), String> {
        self.booster = self.booster.clone().set_allow_missing_splits(value);
        Ok(())
    }

    fn set_create_missing_branch(&mut self, value: bool) -> Result<(), String> {
        self.booster = self.booster.clone().set_create_missing_branch(value);
        Ok(())
    }

    fn set_terminate_missing_features(&mut self, value: HashSet<usize>) -> Result<(), String> {
        self.booster = self.booster.clone().set_terminate_missing_features(value);
        Ok(())
    }

    fn set_missing_node_treatment(&mut self, value: &str) -> Result<(), String> {
        let missing_node_treatment_ = serde_plain::from_str(value).map_err(|e| e.to_string())?;
        self.booster = self.booster.clone().set_missing_node_treatment(missing_node_treatment_);
        Ok(())
    }

    fn set_log_iterations(&mut self, value: usize) -> Result<(), String> {
        self.booster = self.booster.clone().set_log_iterations(value);
        Ok(())
    }

    fn set_quantile(&mut self, value: Option<f64>) -> Result<(), String> {
        self.booster = self.booster.clone().set_quantile(value);
        Ok(())
    }

    fn set_reset(&mut self, value: Option<bool>) -> Result<(), String> {
        self.booster = self.booster.clone().set_reset(value);
        Ok(())
    }

    fn set_categorical_features(&mut self, value: Option<HashSet<usize>>) -> Result<(), String> {
        self.booster = self.booster.clone().set_categorical_features(value);
        Ok(())
    }

    fn set_timeout(&mut self, value: Option<f32>) -> Result<(), String> {
        self.booster = self.booster.clone().set_timeout(value);
        Ok(())
    }

    fn set_iteration_limit(&mut self, value: Option<usize>) -> Result<(), String> {
        self.booster = self.booster.clone().set_iteration_limit(value);
        Ok(())
    }

    fn set_memory_limit(&mut self, value: Option<f32>) -> Result<(), String> {
        self.booster = self.booster.clone().set_memory_limit(value);
        Ok(())
    }

    fn set_stopping_rounds(&mut self, value: Option<usize>) -> Result<(), String> {
        self.booster = self.booster.clone().set_stopping_rounds(value);
        Ok(())
    }

    fn base_score(&self) -> Result<Vec<f64>, String> {
        Ok(self.booster.boosters.iter().map(|b| b.base_score).collect())
    }

    fn number_of_trees(&self) -> Result<Vec<usize>, String> {
        Ok(self.booster.boosters.iter().map(|b| b.get_prediction_trees().len()).collect())
    }

    fn fit(
        &mut self,
        flat_data: Vec<f64>,
        rows: usize,
        cols: usize,
        y: Vec<f64>,
        sample_weight: Option<Vec<f64>>,
    ) -> Result<(), String> {
        let data = Matrix::new(&flat_data, rows, cols);
        let y_data = Matrix::new(&y, rows, self.booster.n_boosters);
        let sample_weight_ = sample_weight.as_ref().map(|sw| sw.as_slice());

        self.booster.fit(&data, &y_data, sample_weight_).map_err(|e| e.to_string())
    }

    fn prune(
        &mut self,
        flat_data: Vec<f64>,
        rows: usize,
        cols: usize,
        y: Vec<f64>,
        sample_weight: Option<Vec<f64>>,
    ) -> Result<(), String> {
        let data = Matrix::new(&flat_data, rows, cols);
        let y_data = Matrix::new(&y, rows, self.booster.n_boosters);
        let sample_weight_ = sample_weight.as_ref().map(|sw| sw.as_slice());

        self.booster.prune(&data, &y_data, sample_weight_).map_err(|e| e.to_string())
    }

    fn predict(
        &self,
        flat_data: Vec<f64>,
        rows: usize,
        cols: usize,
        parallel: Option<bool>,
    ) -> Result<Vec<f64>, String> {
        let data = Matrix::new(&flat_data, rows, cols);
        let parallel = parallel.unwrap_or(true);

        Ok(self.booster.predict(&data, parallel))
    }

    fn predict_proba(
        &self,
        flat_data: Vec<f64>,
        rows: usize,
        cols: usize,
        parallel: Option<bool>,
    ) -> Result<Vec<f64>, String> {
        let data = Matrix::new(&flat_data, rows, cols);
        let parallel = parallel.unwrap_or(true);

        Ok(self.booster.predict_proba(&data, parallel))
    }

    fn save_booster(&self, path: &str) -> Result<(), String> {
        self.booster.save_booster(path).map_err(|e| e.to_string())
    }

    fn json_dump(&self) -> Result<String, String> {
        self.booster.json_dump().map_err(|e| e.to_string())
    }

    fn insert_metadata(&mut self, key: String, value: String) -> Result<(), String> {
        self.booster.insert_metadata(key, value);
        Ok(())
    }

    fn get_metadata(&self, key: String) -> Result<String, String> {
        self.booster.get_metadata(&key).ok_or_else(|| format!("No value associated with provided key {}", key))
    }

    fn load_booster(path: String) -> Result<Self, String> {
        let booster = CrateMultiOutputBooster::load_booster(&path).map_err(|e| e.to_string())?;
        Ok(MultiOutputBooster { booster })
    }

    fn from_json(json_str: &str) -> Result<Self, String> {
        let booster = CrateMultiOutputBooster::from_json(json_str).map_err(|e| e.to_string())?;
        Ok(MultiOutputBooster { booster })
    }

    fn get_params(&self) -> Result<HashMap<&str, Robj>, String> {
        let objective_ = serde_plain::to_string::<Objective>(&self.booster.objective).map_err(|e| e.to_string())?;
        let missing_node_treatment_ = serde_plain::to_string::<MissingNodeTreatment>(&self.booster.missing_node_treatment).map_err(|e| e.to_string())?;
        let monotone_constraints_: HashMap<usize, i8> = self
            .booster
            .monotone_constraints
            .as_ref()
            .unwrap_or(&HashMap::new())
            .iter()
            .map(|(f, c)| {
                let c_ = match c {
                    Constraint::Negative => -1,
                    Constraint::Positive => 1,
                    Constraint::Unconstrained => 0,
                };
                (*f, c_)
            })
            .collect();

        let mut params = HashMap::new();
        params.insert("objective", Robj::from(objective_));
        params.insert("num_threads", Robj::from(self.booster.num_threads));
        params.insert("allow_missing_splits", Robj::from(self.booster.allow_missing_splits));
        params.insert("monotone_constraints", Robj::from(monotone_constraints_));
        params.insert("missing", Robj::from(self.booster.missing));
        params.insert("create_missing_branch", Robj::from(self.booster.create_missing_branch));
        params.insert("terminate_missing_features", Robj::from(self.booster.terminate_missing_features));
        params.insert("missing_node_treatment", Robj::from(missing_node_treatment_));
        params.insert("log_iterations", Robj::from(self.booster.log_iterations));
        params.insert("force_children_to_bound_parent", Robj::from(self.booster.force_children_to_bound_parent));
        params.insert("quantile", Robj::from(self.booster.quantile));
        params.insert("reset", Robj::from(self.booster.reset));
        params.insert("categorical_features", Robj::from(self.booster.categorical_features));
        params.insert("timeout", Robj::from(self.booster.timeout));
        params.insert("iteration_limit", Robj::from(self.booster.iteration_limit));
        params.insert("memory_limit", Robj::from(self.booster.memory_limit));
        params.insert("stopping_rounds", Robj::from(self.booster.stopping_rounds));

        Ok(params)
    }
}

extendr_module! {
    mod perpetual;
    impl MultiOutputBooster;
}
