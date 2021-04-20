use crate::data::elements::matrix::{Matrix, MatrixValue};
use std::sync::{Arc, RwLock};
use traversal::Bft;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiveMatrix {
    pub children_groups: Vec<Vec<Self>>,
    pub value: MatrixValue,
}

impl LiveMatrix {
    pub fn level_order_traversal(m: LiveMatrix) -> Vec<(usize, MatrixValue)> {
        let iter = Bft::new(&m, |node| node.children_groups.iter().flatten());
        let mut iter = iter.map(|(depth, node)| (depth, &node.value));
        iter.map(|s| (s.0, s.1.clone())).collect::<Vec<_>>()
    }

    pub fn from_matrix(m: &Matrix) -> Self {
        let mut children_groups = vec![];

        for (gidx, g) in m.children_groups.iter().enumerate() {
            let mut buff = vec![];

            for (idx, i) in g.iter().enumerate() {
                let item = arc!(i);
                let live = LiveMatrix::from_matrix(&*item.read().unwrap());
                buff.push(live)
            }

            if !buff.is_empty() {
                children_groups.push(buff);
            }
        }

        Self {
            children_groups,
            value: m.value.clone(),
        }
    }
}
