use crate::data::elements::matrix::Matrix;
use std::sync::{Arc, RwLock};

impl Matrix {
    pub fn remove_matrix(
        target: Arc<RwLock<Matrix>>,
        root: Arc<RwLock<Matrix>>,
    ) -> Result<Arc<RwLock<Matrix>>, String> {
        let target_id = target.read().unwrap().value.id.clone();

        if let Some(parent) = target.read().unwrap().value.parent_matrix.clone() {
            match Matrix::get_by_id(parent.id, arc!(root)) {
                Ok(item) => {
                    let mut parent_matrix = item.write().unwrap();
                    parent_matrix
                        .children_groups
                        .iter_mut()
                        .map(|s| s.retain(|s| s.read().unwrap().value.id != target_id));
                }
                Err(msg) => {
                    return Err(format!("searched: {} nodes", &msg));
                }
            }
        } else {
            return Err(format!("No parent found"));
        }

        Ok(target)
    }
}
