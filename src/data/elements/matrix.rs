use crate::constant::utility::UUID_PLACEHOLDER;
use crate::data::elements::landing_page::LandingPage;
use crate::data::elements::offer::Offer;
use std::borrow::{Borrow, BorrowMut};
use std::cell::RefCell;
use std::error::Error;
use std::rc::Rc;
use std::sync::{Arc, RwLock};
use traversal::{Bft, DftLongestPaths};
use uuid::Uuid;
pub mod live_matrix;
pub mod remove;

impl PartialEq for Matrix {
    fn eq(&self, other: &Self) -> bool {
        self.value.id == other.value.id
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Matrix {
    pub children_groups: Vec<Vec<Arc<RwLock<Self>>>>,
    pub value: MatrixValue,
}

impl Matrix {
    pub fn get_all_paths(
        root: Arc<RwLock<Matrix>>,
        mut list: Option<Vec<Arc<RwLock<Matrix>>>>,
    ) -> Vec<Arc<RwLock<Matrix>>> {
        let mut paths = vec![];

        root.read()
            .unwrap()
            .children_groups
            .iter()
            .flatten()
            .map(|s| {
                paths.push(arc!(s));
                paths = Matrix::get_all_paths(arc!(s), Some(paths.clone()));
            });

        paths
    }

    pub fn get_by_id(id: Uuid, matrix: Arc<RwLock<Matrix>>) -> Result<Arc<RwLock<Matrix>>, String> {
        let matrix = arc!(matrix);
        // let groups = matrix.read().unwrap().children_groups.iter().enumerate();

        // if !groups.is_empty() {
        for (gi, g) in matrix.read().unwrap().children_groups.iter().enumerate() {
            for (idx, i) in g.iter().enumerate() {
                let local_matrix = arc!(i);
                let lid = local_matrix.read().unwrap().value.id.clone();
                // let children = local_matrix
                //     .read()
                //     .unwrap()
                //     .children_groups
                //     .iter()
                //     .flatten();

                if lid == id {
                    return Ok(arc!(local_matrix));
                } else {
                    for i in local_matrix
                        .read()
                        .unwrap()
                        .children_groups
                        .iter()
                        .flatten()
                    {
                        let child = arc!(i);
                        if let Ok(found) = Matrix::get_by_id(id.clone(), child) {
                            return Ok(found);
                        }
                    }
                }
            }
        }
        // } else {
        Err(format!("Not found..."))
        //
        // }

        // let res = ;

        // if let Some(found) = matrix
        //     .read()
        //     .unwrap()
        //     .children_groups
        //     .iter()
        //     .flatten()
        //     .find(|s| s.read().unwrap().value.id == id)
        // {
        //     return Ok(arc!(found));
        // }

        //
        // for child in matrix.read().unwrap().children_groups.iter().flatten() {
        //     if let Ok(child) = Matrix::get_by_id(id.clone(), arc!(child)) {
        //         return Ok(child);
        //     }
        // }

        // let mut children = vec![];
        // let mut error_count = 0usize;
        //
        // for (group_idx, group) in matrix.read().unwrap().children_groups.iter().enumerate() {
        //     for (idx, item) in group.iter().enumerate() {
        //         let item_id = item.read().unwrap().value.id.clone();
        //
        //         if item_id == id {
        //             return Ok(arc!(item));
        //         } else {
        //             if !item.read().unwrap().children_groups.is_empty() {
        //                 item.read()
        //                     .unwrap()
        //                     .children_groups
        //                     .iter()
        //                     .flatten()
        //                     .map(|s| children.push(arc!(s)));
        //             }
        //         }
        //
        //         error_count + 1;
        //     }
        // }
        //
        // if !children.is_empty() {
        //     for child in children {
        //         match Matrix::get_by_id(id.clone(), child) {
        //             Ok(x) => return Ok(x),
        //             Err(msg) => {
        //                 error_count + msg;
        //             }
        //         }
        //     }
        // }
        //
        // Err(error_count)
    }

    pub fn fix_idx_pos(matrix: Arc<RwLock<Matrix>>) -> Arc<RwLock<Matrix>> {
        for (group_idx, group) in matrix.read().unwrap().children_groups.iter().enumerate() {
            for (idx, item) in group.iter().enumerate() {
                let iidx = item.read().unwrap().value.item_idx;
                let gidx = item.read().unwrap().value.group_idx;

                if iidx != idx || gidx != group_idx {
                    item.write().unwrap().value.item_idx = idx;
                    item.write().unwrap().value.group_idx = group_idx;
                }

                Matrix::fix_idx_pos(arc!(item));
            }
        }

        matrix
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatrixValue {
    pub id: Uuid,
    pub parent_matrix: Option<Box<MatrixValue>>,
    pub group_idx: usize,
    pub item_idx: usize,
    pub depth: usize,
    pub data: MatrixData,
}

impl PartialEq for MatrixValue {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.data == other.data
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MatrixData {
    Offer(Offer),
    LandingPage(LandingPage),
    Source,
    Void,
}

impl MatrixValue {
    pub fn child_depth(&self) -> usize {
        self.depth + 1
    }
}

impl Matrix {
    pub fn new_item_idx(&self, from_group_idx: usize) -> Option<usize> {
        if let Some(g) = self.children_groups.get(from_group_idx) {
            Some(g.len())
        } else {
            None
        }
    }

    pub fn new_group_idx(&self) -> usize {
        self.children_groups.len()
    }

    pub fn synchronize_matrix_child_groups(target: Arc<RwLock<Self>>) -> Result<(), String> {
        let mut matrix_handle = target.write().expect("FG$%sdfg");
        let next_depth = matrix_handle.depth() + 1;

        if let MatrixData::LandingPage(lp) = matrix_handle.data() {
            let total_child_groups = matrix_handle.children_groups.len();
            let num_of_ctas = lp.number_of_calls_to_action;
            if total_child_groups == num_of_ctas as usize {
                return Ok(());
            } else if total_child_groups > num_of_ctas as usize {
                for i in (num_of_ctas as usize..total_child_groups).rev() {
                    matrix_handle.children_groups.remove(i);
                }
            } else if total_child_groups < num_of_ctas as usize {
                for i in total_child_groups..num_of_ctas as usize {
                    matrix_handle
                        .children_groups
                        .push(vec![Arc::new(RwLock::new(Matrix::void(
                            Some(Box::new(target.read().unwrap().value.clone())),
                            i,
                            0,
                            next_depth,
                        )))])
                }
            }

            if let MatrixData::LandingPage(lp) = matrix_handle.data() {
                if matrix_handle.children_groups.len() == lp.number_of_calls_to_action as usize {
                    Ok(())
                } else {
                    Err("Synchronize Failed: g54sdfg".to_string())
                }
            } else {
                Err(String::from("Not a landing page"))
            }
        } else {
            Err(String::from("Not a landing page"))
        }
    }

    pub fn highest_cta(target_group: &Vec<Arc<RwLock<Matrix>>>) -> usize {
        let mut highest = 0usize;
        for i in target_group {
            if let MatrixData::LandingPage(lp) = &i.read().unwrap().value.data {
                if lp.number_of_calls_to_action as usize > highest {
                    highest = lp.number_of_calls_to_action as usize;
                }
            }
        }
        highest
    }

    pub fn root_synchronize_landing_page_child_groups(
        target: Arc<RwLock<Matrix>>,
    ) -> Result<(), String> {
        let parent_node = arc!(target);
        let mut matrix_handle = target.write().expect("FG$%sdfg");
        let next_depth = matrix_handle.depth() + 1;

        let mut max_num_ctas = 0usize;
        let mut num_offer_groups = 0usize;

        for item in matrix_handle.children_groups.get(0).unwrap() {
            let item_handle = item.read().expect("^GH%fsd");

            if let MatrixData::LandingPage(lp) = item_handle.data() {
                if lp.number_of_calls_to_action as usize > max_num_ctas {
                    max_num_ctas = lp.number_of_calls_to_action as usize;
                }
            }
        }

        Ok(())
    }

    pub fn item_idx(&self) -> usize {
        self.value.item_idx
    }

    pub fn group_idx(&self) -> usize {
        self.value.group_idx
    }

    pub fn id(&self) -> &Uuid {
        &self.value.id
    }

    pub fn data(&self) -> &MatrixData {
        &self.value.data
    }

    pub fn depth(&self) -> usize {
        self.value.depth
    }

    pub fn source() -> Arc<RwLock<Self>> {
        let mut matrix = Arc::new(RwLock::new(Self {
            children_groups: vec![],
            value: MatrixValue {
                id: Uuid::new_v4(),
                parent_matrix: None,
                group_idx: 0,
                item_idx: 0,
                depth: 0,
                data: MatrixData::Source,
            },
        }));

        let parent_matrix = Some(Box::new(matrix.read().unwrap().value.clone()));

        matrix
            .write()
            .expect("G%FDfg")
            .children_groups
            .push(vec![Arc::new(RwLock::new(Matrix::void(
                parent_matrix,
                0,
                0,
                1,
            )))]);

        matrix
    }

    pub fn sync_item_idx(group_idx: usize, matrix: Arc<RwLock<Matrix>>) -> Result<(), String> {
        if let Ok(read) = matrix.read() {
            if let Some(group) = read.children_groups.get(group_idx) {
                for (item_idx, item) in group.iter().enumerate() {
                    if let Ok(mut local_handle) = item.write() {
                        local_handle.value.item_idx = item_idx;
                    } else {
                        return Err(format!("Write Lock Err: {}", item_idx));
                    }
                }
                Ok(())
            } else {
                return Err("No Group Idx".to_string());
            }
        } else {
            return Err("Read Lock Err".to_string());
        }
    }

    pub fn transform_void(target: Arc<RwLock<Matrix>>, new: Transform) {
        let local_matrix = arc!(target);
        let mut matrix_handle = target.write().expect("G^%FRFe0");

        match new {
            Transform::Offer(o) => matrix_handle.value.data = MatrixData::Offer(o),
            Transform::Lander(lp) => {
                let ctas = lp.number_of_calls_to_action;
                matrix_handle.value.data = MatrixData::LandingPage(lp);
                Matrix::root_synchronize_landing_page_child_groups(local_matrix);
            }
        }
    }

    pub fn void(
        parent_matrix: Option<Box<MatrixValue>>,
        group_idx: usize,
        item_idx: usize,
        depth: usize,
    ) -> Self {
        let id = Uuid::new_v4();

        Self {
            children_groups: vec![],
            value: MatrixValue {
                id,
                parent_matrix,
                group_idx,
                item_idx,
                depth,
                data: MatrixData::Void,
            },
        }
    }
}

pub enum Transform {
    Offer(Offer),
    Lander(LandingPage),
}
