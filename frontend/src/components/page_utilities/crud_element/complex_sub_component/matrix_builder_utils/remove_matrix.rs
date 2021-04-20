use crate::notify_danger;
use ad_buy_engine::data::elements::funnel::SequenceType;
use ad_buy_engine::data::elements::matrix::{Matrix, MatrixData};
use std::sync::{Arc, RwLock};

pub fn remove_matrix(
    seq_type: &SequenceType,
    local_matrix: Arc<RwLock<Matrix>>,
) -> Result<(), String> {
    match seq_type {
        SequenceType::Matrix => {
            let parent = arc!(local_matrix
                .read()
                .unwrap()
                .value
                .parent_matrix
                .as_ref()
                .expect("%GRdcfgs"));
            let local_group_idx = local_matrix.read().unwrap().value.group_idx;
            let local_matrix_id = local_matrix.read().unwrap().value.id;
            let parent_children_len = parent.read().unwrap().children_groups.len();

            let mut parent_matrix = parent.write().unwrap();

            if let MatrixData::Source = parent_matrix.value.data {
                if parent_matrix.children_groups.len() == 1 {
                    parent_matrix.children_groups.clear();
                } else {
                    parent_matrix.children_groups.remove(local_group_idx);

                    for (idx, i) in parent_matrix.children_groups.iter().enumerate() {
                        if let Some(item) = i.get(0) {
                            let mut matrix = item.write().unwrap();
                            if matrix.value.group_idx != idx {
                                matrix.value.group_idx = idx;
                            }
                        }
                    }
                }
            } else {
                if let Some(group) = parent_matrix.children_groups.get_mut(local_group_idx) {
                    group.retain(|s| s.read().unwrap().value.id != local_matrix_id)
                    // if group.len() == 1 {
                    //     notify_danger("1");
                    //     group.get(0).unwrap().write().unwrap().value.data = MatrixData::Void;
                    // } else {
                    //     notify_danger("2");
                    //     group.retain(|s| s.read().unwrap().value.id != local_matrix_id);
                    // }
                }
            }

            Ok(())
        }

        SequenceType::LandingPageAndOffers => return Err(new_string!("not setup")),

        SequenceType::OffersOnly => {
            let target_item_id = local_matrix.read().expect("g5rtsfdgF").value.id.clone();
            let target_item_idx = local_matrix.read().expect("g5rtsfdgF").value.item_idx;
            let target_group_idx = local_matrix.read().expect("g5rtsfdgF").group_idx();

            if let Some(parent) = &local_matrix.read().unwrap().value.parent_matrix {
                if let Some(group) = parent
                    .write()
                    .unwrap()
                    .children_groups
                    .get_mut(target_group_idx)
                {
                    let res = group.retain(|s| s.read().unwrap().value.id != target_item_id);
                    return Ok(());
                } else {
                    return Err(new_string!("No Group Found"));
                }
            } else {
                return Err(new_string!("No Parent"));
            }
        }
    }
}
