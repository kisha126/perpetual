use crate::utils::int_map_to_constraint_map;
use extendr_api::prelude::*;
use perpetual_rs::booster::booster::MissingNodeTreatment;
use perpetual_rs::booster::booster::PerpetualBooster as CratePerpetualBooster;
use perpetual_rs::conformal::cqr::CalData;
use perpetual_rs::constraints::Constraint;
use perpetual_rs::data::Matrix;
use perpetual_rs::objective::Objective;
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub struct PerpetualBooster {
    booster: CratePerpetualBooster,
}

#[extendr]
impl PerpetualBooster {
    fn new(
        objective: &str,
        budget: f32,
        max_bin: u16,
        num_threads: Robj,
        monotone_constraints: Robj,
        force_children_to_bound_parent: bool,
        missing: f64,
        allow_missing_splits: bool,
        create_missing_branch: bool,
        terminate_missing_features: Robj,
        missing_node_treatment: &str,
        log_iterations: usize,
        quantile: Robj,
        reset: Robj,
        categorical_features: Robj,
        timeout: Robj,
        iteration_limit: Robj,
        memory_limit: Robj,
        stopping_rounds: Robj,
    ) -> Result<Robj> {
        // Convert R optional values to Rust Option
        let num_threads_opt: Option<usize> = if num_threads.is_null() { 
            None 
        } else { 
            Some(num_threads.as_usize()?) 
        };
        
        // Convert R named list to HashMap
        let monotone_map: HashMap<usize, i8> = if monotone_constraints.is_list() {
            let names = monotone_constraints.names().unwrap();
            let mut map = HashMap::new();
            for (i, name) in names.iter().enumerate() {
                let key: usize = name.parse().map_err(|_| Error::Other("Invalid feature index".into()))?;
                let value_obj = monotone_constraints.get_list_element(i)?;
                let value: i8 = value_obj.as_integer()? as i8;
                map.insert(key, value);
            }
            map
        } else {
            HashMap::new()
        };
        
        // Convert R vector to HashSet
        let terminate_missing: HashSet<usize> = if terminate_missing_features.is_vector() {
            let vec = terminate_missing_features.as_integer_vector()?;
            vec.iter().map(|&x| x as usize).collect()
        } else {
            HashSet::new()
        };
        
        // Convert remaining optional parameters
        let quantile_opt: Option<f64> = if quantile.is_null() { None } else { Some(quantile.as_real()?) };
        let reset_opt: Option<bool> = if reset.is_null() { None } else { Some(reset.as_bool()?) };
        let timeout_opt: Option<f32> = if timeout.is_null() { None } else { Some(timeout.as_real()? as f32) };
        let iteration_limit_opt: Option<usize> = if iteration_limit.is_null() { None } else { Some(iteration_limit.as_usize()?) };
        let memory_limit_opt: Option<f32> = if memory_limit.is_null() { None } else { Some(memory_limit.as_real()? as f32) };
        let stopping_rounds_opt: Option<usize> = if stopping_rounds.is_null() { None } else { Some(stopping_rounds.as_usize()?) };
        
        // Convert categorical features
        let categorical_features_opt: Option<HashSet<usize>> = if categorical_features.is_null() {
            None
        } else {
            let vec = categorical_features.as_integer_vector()?;
            Some(vec.iter().map(|&x| x as usize).collect())
        };
        
        // Parse string enums
        let objective_: Objective = serde_plain::from_str(objective)
            .map_err(|e| Error::Other(format!("Invalid objective: {}", e)))?;
            
        let missing_node_treatment_: MissingNodeTreatment = serde_plain::from_str(missing_node_treatment)
            .map_err(|e| Error::Other(format!("Invalid missing_node_treatment: {}", e)))?;
            
        let monotone_constraints_ = int_map_to_constraint_map(monotone_map)
            .map_err(|e| Error::Other(format!("Invalid monotone constraints: {}", e)))?;
        
        let booster = CratePerpetualBooster::default()
            .set_objective(objective_)
            .set_budget(budget)
            .set_max_bin(max_bin)
            .set_num_threads(num_threads_opt)
            .set_monotone_constraints(Some(monotone_constraints_))
            .set_force_children_to_bound_parent(force_children_to_bound_parent)
            .set_missing(missing)
            .set_allow_missing_splits(allow_missing_splits)
            .set_create_missing_branch(create_missing_branch)
            .set_terminate_missing_features(terminate_missing)
            .set_missing_node_treatment(missing_node_treatment_)
            .set_log_iterations(log_iterations)
            .set_quantile(quantile_opt)
            .set_reset(reset_opt)
            .set_categorical_features(categorical_features_opt)
            .set_timeout(timeout_opt)
            .set_iteration_limit(iteration_limit_opt)
            .set_memory_limit(memory_limit_opt)
            .set_stopping_rounds(stopping_rounds_opt);
        
        // Validate parameters
        booster.validate_parameters()
            .map_err(|e| Error::Other(format!("Invalid parameters: {}", e)))?;
        
        Ok(PerpetualBooster { booster }.into_robj()?)
    }
    
    fn set_objective(&mut self, value: &str) -> Result<()> {
        let objective_ = serde_plain::from_str(value)
            .map_err(|e| Error::Other(format!("Invalid objective: {}", e)))?;
        self.booster.objective = objective_;
        Ok(())
    }
    
    fn set_budget(&mut self, value: f32) -> Result<()> {
        self.booster.budget = value;
        Ok(())
    }
    
    fn set_max_bin(&mut self, value: u16) -> Result<()> {
        self.booster.max_bin = value;
        Ok(())
    }
    
    fn set_num_threads(&mut self, value: Robj) -> Result<()> {
        self.booster.num_threads = if value.is_null() { 
            None 
        } else { 
            Some(value.as_usize()?) 
        };
        Ok(())
    }
    
    fn set_monotone_constraints(&mut self, value: Robj) -> Result<()> {
        let monotone_map: HashMap<usize, i8> = if value.is_list() {
            let names = value.names().unwrap();
            let mut map = HashMap::new();
            for (i, name) in names.iter().enumerate() {
                let key: usize = name.parse().map_err(|_| Error::Other("Invalid feature index".into()))?;
                let value_obj = value.get_list_element(i)?;
                let value: i8 = value_obj.as_integer()? as i8;
                map.insert(key, value);
            }
            map
        } else {
            HashMap::new()
        };
        
        let map = int_map_to_constraint_map(monotone_map)
            .map_err(|e| Error::Other(format!("Invalid monotone constraints: {}", e)))?;
        self.booster.monotone_constraints = Some(map);
        Ok(())
    }
    
    fn set_force_children_to_bound_parent(&mut self, value: bool) -> Result<()> {
        self.booster.force_children_to_bound_parent = value;
        Ok(())
    }
    
    fn set_missing(&mut self, value: f64) -> Result<()> {
        self.booster.missing = value;
        Ok(())
    }
    
    fn set_allow_missing_splits(&mut self, value: bool) -> Result<()> {
        self.booster.allow_missing_splits = value;
        Ok(())
    }
    
    fn set_create_missing_branch(&mut self, value: bool) -> Result<()> {
        self.booster.create_missing_branch = value;
        Ok(())
    }
    
    fn set_terminate_missing_features(&mut self, value: Robj) -> Result<()> {
        let terminate_missing: HashSet<usize> = if value.is_vector() {
            let vec = value.as_integer_vector()?;
            vec.iter().map(|&x| x as usize).collect()
        } else {
            HashSet::new()
        };
        
        self.booster.terminate_missing_features = terminate_missing;
        Ok(())
    }
    
    fn set_missing_node_treatment(&mut self, value: &str) -> Result<()> {
        let missing_node_treatment_ = serde_plain::from_str(value)
            .map_err(|e| Error::Other(format!("Invalid missing_node_treatment: {}", e)))?;
        self.booster.missing_node_treatment = missing_node_treatment_;
        Ok(())
    }
    
    fn set_log_iterations(&mut self, value: usize) -> Result<()> {
        self.booster.log_iterations = value;
        Ok(())
    }
    
    fn set_quantile(&mut self, value: Robj) -> Result<()> {
        self.booster.quantile = if value.is_null() { None } else { Some(value.as_real()?) };
        Ok(())
    }
    
    fn set_reset(&mut self, value: Robj) -> Result<()> {
        self.booster.reset = if value.is_null() { None } else { Some(value.as_bool()?) };
        Ok(())
    }
    
    fn set_categorical_features(&mut self, value: Robj) -> Result<()> {
        self.booster.categorical_features = if value.is_null() {
            None
        } else {
            let vec = value.as_integer_vector()?;
            Some(vec.iter().map(|&x| x as usize).collect())
        };
        Ok(())
    }
    
    fn set_timeout(&mut self, value: Robj) -> Result<()> {
        self.booster.timeout = if value.is_null() { None } else { Some(value.as_real()? as f32) };
        Ok(())
    }
    
    fn set_iteration_limit(&mut self, value: Robj) -> Result<()> {
        self.booster.iteration_limit = if value.is_null() { None } else { Some(value.as_usize()?) };
        Ok(())
    }
    
    fn set_memory_limit(&mut self, value: Robj) -> Result<()> {
        self.booster.memory_limit = if value.is_null() { None } else { Some(value.as_real()? as f32) };
        Ok(())
    }
    
    fn set_stopping_rounds(&mut self, value: Robj) -> Result<()> {
        self.booster.stopping_rounds = if value.is_null() { None } else { Some(value.as_usize()?) };
        Ok(())
    }
    
    fn base_score(&self) -> f64 {
        self.booster.base_score
    }
    
    fn number_of_trees(&self) -> usize {
        self.booster.get_prediction_trees().len()
    }
    
    fn fit(&mut self, flat_data: &[f64], rows: usize, cols: usize, y: &[f64], sample_weight: Robj) -> Result<()> {
        let data = Matrix::new(flat_data, rows, cols);
        
        let sample_weight_: Option<&[f64]> = if sample_weight.is_null() {
            None
        } else {
            let sw_vec = sample_weight.as_real_vector()?;
            Some(sw_vec.as_slice())
        };
        
        self.booster.fit(&data, y, sample_weight_)
            .map_err(|e| Error::Other(format!("Fit error: {}", e)))?;
            
        Ok(())
    }
    
    fn prune(&mut self, flat_data: &[f64], rows: usize, cols: usize, y: &[f64], sample_weight: Robj) -> Result<()> {
        let data = Matrix::new(flat_data, rows, cols);
        
        let sample_weight_: Option<&[f64]> = if sample_weight.is_null() {
            None
        } else {
            let sw_vec = sample_weight.as_real_vector()?;
            Some(sw_vec.as_slice())
        };
        
        self.booster.prune(&data, y, sample_weight_)
            .map_err(|e| Error::Other(format!("Prune error: {}", e)))?;
            
        Ok(())
    }
    
    fn calibrate(
        &mut self, 
        flat_data: &[f64], 
        rows: usize, 
        cols: usize, 
        y: &[f64], 
        flat_data_cal: &[f64], 
        rows_cal: usize, 
        cols_cal: usize, 
        y_cal: &[f64], 
        alpha: &[f64], 
        sample_weight: Robj
    ) -> Result<()> {
        let data = Matrix::new(flat_data, rows, cols);
        let data_cal = Matrix::new(flat_data_cal, rows_cal, cols_cal);
        
        let sample_weight_: Option<&[f64]> = if sample_weight.is_null() {
            None
        } else {
            let sw_vec = sample_weight.as_real_vector()?;
            Some(sw_vec.as_slice())
        };
        
        let cal_data: CalData = (data_cal, y_cal, alpha);
        
        self.booster.calibrate(&data, y, sample_weight_, cal_data)
            .map_err(|e| Error::Other(format!("Calibrate error: {}", e)))?;
            
        Ok(())
    }
    
    fn predict_intervals(&self, flat_data: &[f64], rows: usize, cols: usize, parallel: Robj) -> Robj {
        let data = Matrix::new(flat_data, rows, cols);
        let parallel_opt = if parallel.is_null() { true } else { parallel.as_bool().unwrap() };
        
        let predictions = self.booster.predict_intervals(&data, parallel_opt);
        
        // Convert to R list
        let r_list = R!("list()")?;
        for (key, value) in predictions.iter() {
            let array = value.clone().into_iter().flatten().collect::<Vec<f64>>();
            let dim = r!(c(value.len(), value[0].len()))?;
            let r_array = r!(array(array, dim))?;
            r_list.set_list_element(key, r_array)?;
        }
        
        r_list
    }
    
    fn predict(&self, flat_data: &[f64], rows: usize, cols: usize, parallel: Robj) -> Vec<f64> {
        let data = Matrix::new(flat_data, rows, cols);
        let parallel_opt = if parallel.is_null() { true } else { parallel.as_bool().unwrap() };
        
        self.booster.predict(&data, parallel_opt)
    }
    
    fn predict_proba(&self, flat_data: &[f64], rows: usize, cols: usize, parallel: Robj) -> Vec<f64> {
        let data = Matrix::new(flat_data, rows, cols);
        let parallel_opt = if parallel.is_null() { true } else { parallel.as_bool().unwrap() };
        
        self.booster.predict_proba(&data, parallel_opt)
    }
    
    fn predict_contributions(&self, flat_data: &[f64], rows: usize, cols: usize, method: &str, parallel: Robj) -> Vec<f64> {
        let data = Matrix::new(flat_data, rows, cols);
        let parallel_opt = if parallel.is_null() { true } else { parallel.as_bool().unwrap() };
        
        let method_ = serde_plain::from_str(method).unwrap();
        
        self.booster.predict_contributions(&data, method_, parallel_opt)
    }
    
    fn calculate_feature_importance(&self, method: &str, normalize: bool) -> Robj {
        let method_ = serde_plain::from_str(method).unwrap();
        let importance_map = self.booster.calculate_feature_importance(method_, normalize);
        
        // Convert to named R vector
        let values: Vec<f32> = importance_map.values().cloned().collect();
        let names: Vec<String> = importance_map.keys().map(|k| k.to_string()).collect();
        
        let r_vec = RVector::from(values);
        r_vec.set_names(names).unwrap();
        
        r_vec.into_robj()
    }
    
    fn value_partial_dependence(&self, feature: usize, value: f64) -> f64 {
        self.booster.value_partial_dependence(feature, value)
    }
    
    fn text_dump(&self) -> Vec<String> {
        self.booster.trees.iter()
            .map(|t| format!("{}", t))
            .collect()
    }
    
    fn save_booster(&self, path: &str) -> Result<()> {
        self.booster.save_booster(path)
            .map_err(|e| Error::Other(format!("Save error: {}", e)))
    }
    
    fn json_dump(&self) -> Result<String> {
        self.booster.json_dump()
            .map_err(|e| Error::Other(format!("JSON dump error: {}", e)))
    }
    
    fn insert_metadata(&mut self, key: &str, value: &str) -> Result<()> {
        self.booster.insert_metadata(key.to_string(), value.to_string());
        Ok(())
    }
    
    fn get_metadata(&self, key: &str) -> Result<String> {
        match self.booster.get_metadata(key) {
            Some(m) => Ok(m),
            None => Err(Error::Other(format!("No value associated with provided key {}", key))),
        }
    }
    
    fn load_booster(path: &str) -> Result<Robj> {
        let booster = CratePerpetualBooster::load_booster(path)
            .map_err(|e| Error::Other(format!("Load error: {}", e)))?;
            
        Ok(PerpetualBooster { booster }.into_robj()?)
    }
    
    fn from_json(json_str: &str) -> Result<Robj> {
        let booster = CratePerpetualBooster::from_json(json_str)
            .map_err(|e| Error::Other(format!("JSON parse error: {}", e)))?;
            
        Ok(PerpetualBooster { booster }.into_robj()?)
    }
    
    fn get_params(&self) -> Robj {
        let objective_ = serde_plain::to_string::<Objective>(&self.booster.objective).unwrap();
        let missing_node_treatment_ = serde_plain::to_string::<MissingNodeTreatment>(&self.booster.missing_node_treatment).unwrap();
        
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
            
        // Convert to R list
        let r_list = R!("list()")?;
        
        r_list.set_list_element("objective", objective_)?;
        r_list.set_list_element("budget", self.booster.budget)?;
        
        if let Some(nt) = self.booster.num_threads {
            r_list.set_list_element("num_threads", nt)?;
        } else {
            r_list.set_list_element("num_threads", R_NilValue)?;
        }
        
        r_list.set_list_element("allow_missing_splits", self.booster.allow_missing_splits)?;
        
        // Convert monotone constraints to named vector
        let mc_values: Vec<i8> = monotone_constraints_.values().cloned().collect();
        let mc_names: Vec<String> = monotone_constraints_.keys().map(|k| k.to_string()).collect();
        let mc_vec = RVector::from(mc_values);
        mc_vec.set_names(mc_names).unwrap();
        r_list.set_list_element("monotone_constraints", mc_vec)?;
        
        r_list.set_list_element("missing", self.booster.missing)?;
        r_list.set_list_element("create_missing_branch", self.booster.create_missing_branch)?;
        
        // Convert terminate_missing_features to vector
        let tmf_vec = RVector::from(
            self.booster.terminate_missing_features.iter()
                .cloned()
                .collect::<Vec<usize>>()
        );
        r_list.set_list_element("terminate_missing_features", tmf_vec)?;
        
        r_list.set_list_element("missing_node_treatment", missing_node_treatment_)?;
        r_list.set_list_element("log_iterations", self.booster.log_iterations)?;
        r_list.set_list_element("force_children_to_bound_parent", self.booster.force_children_to_bound_parent)?;
        
        if let Some(q) = self.booster.quantile {
            r_list.set_list_element("quantile", q)?;
        } else {
            r_list.set_list_element("quantile", R_NilValue)?;
        }
        
        if let Some(r) = self.booster.reset {
            r_list.set_list_element("reset", r)?;
        } else {
            r_list.set_list_element("reset", R_NilValue)?;
        }
        
        if let Some(ref cf) = self.booster.categorical_features {
            let cf_vec = RVector::from(cf.iter().cloned().collect::<Vec<usize>>());
            r_list.set_list_element("categorical_features", cf_vec)?;
        } else {
            r_list.set_list_element("categorical_features", R_NilValue)?;
        }
        
        if let Some(t) = self.booster.timeout {
            r_list.set_list_element("timeout", t)?;
        } else {
            r_list.set_list_element("timeout", R_NilValue)?;
        }
        
        if let Some(il) = self.booster.iteration_limit {
            r_list.set_list_element("iteration_limit", il)?;
        } else {
            r_list.set_list_element("iteration_limit", R_NilValue)?;
        }
        
        if let Some(ml) = self.booster.memory_limit {
            r_list.set_list_element("memory_limit", ml)?;
        } else {
            r_list.set_list_element("memory_limit", R_NilValue)?;
        }
        
        if let Some(sr) = self.booster.stopping_rounds {
            r_list.set_list_element("stopping_rounds", sr)?;
        } else {
            r_list.set_list_element("stopping_rounds", R_NilValue)?;
        }
        
        r_list
    }
}
