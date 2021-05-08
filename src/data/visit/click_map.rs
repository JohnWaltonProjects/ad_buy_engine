use crate::data::elements::campaign::Campaign;
use crate::data::elements::funnel::{ConditionalSequence, Sequence, SequenceType};
use crate::data::elements::matrix::live_matrix::LiveMatrix;
use crate::data::elements::matrix::{Matrix, MatrixData, MatrixValue};
use crate::data::lists::condition::{Condition, ConditionDataType};
use crate::data::visit::geo_ip::GeoIPData;
use crate::data::visit::user_agent::UserAgentData;
use either::Either;
use std::collections::HashMap;
use std::net::IpAddr;
use std::sync::{Arc, RwLock};
use traversal::{Bft, DftLongestPaths};
use url::Url;
use uuid::Uuid;
use weighted_rs::{SmoothWeight, Weight};
pub mod qualify_condition;
use crate::data::visit::click_event::{ClickEvent, ClickableElement, TerseElement};
use crate::data::visit::click_map::qualify_condition::condi_qualify;
use qualify_condition::qualify_condition;
use serde::de::Unexpected::Seq;
// #[derive(Serialize, Deserialize, Clone, Debug)]
// pub struct OfferClickMap {
//     pub offer_id: Uuid,
//     pub offer_url: Url,
// }
//
// #[derive(Serialize, Deserialize, Clone, Debug)]
// pub struct LandingPageClickMap {
//     pub landing_page_id: Uuid,
//     pub landing_page_url: Url,
//     pub offers: Vec<OfferClickMap>,
// }
//
// #[derive(Serialize, Deserialize, Clone, Debug)]
// pub struct PreLandingPageClickMap {
//     pub landing_page_id: Uuid,
//     pub pre_landing_page_url: Url,
//     pub landing_pages: Vec<LandingPageClickMap>,
// }

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ClickMap {
    pub children: Vec<ClickMap>,
    pub value: MatrixValue,
    pub seq_type: Option<SequenceType>,
    pub linked_conversion_id: Option<String>,
}

pub fn select_child(group: &Vec<LiveMatrix>) -> LiveMatrix {
    if group.len() == 1 {
        group.first().expect("g53sdfg").clone()
    } else {
        let mut sw: SmoothWeight<usize> = SmoothWeight::new();

        for (idx, i) in group.iter().enumerate() {
            let weight = match &i.value.data {
                MatrixData::LandingPage(lp) => lp.weight as isize,
                MatrixData::Offer(lp) => lp.weight as isize,
                _ => 0isize,
            };
            sw.add(idx, weight);
        }

        let selected_idx = sw.next().expect("G%dfs");
        let selected = group.get(selected_idx).expect("T%Gsdf").clone();

        selected
    }
}

impl ClickMap {
    pub fn find_node_in_matrix(&self, mid: Uuid) -> Self {
        let iter = Bft::new(self, |node| node.children.iter());
        let mut iter = iter.map(|(depth, node)| (depth, node));
        let res = iter.find(|(d, n)| n.value.id == mid);
        res.unwrap().1.clone()
    }

    pub fn get_initial_click(&self) -> Result<(Url, ClickEvent), String> {
        match self.seq_type.expect("GTsfd") {
            SequenceType::Offers => {
                if let MatrixData::Offer(offer) = &self.value.data {
                    let click_event = ClickEvent::create(ClickableElement::Offer(
                        TerseElement::new(offer.offer_id.clone(), None),
                    ));
                    Ok((offer.url.clone(), click_event))
                } else {
                    Err("matrix data not offer".to_string())
                }
            }

            _ => {
                if let MatrixData::LandingPage(landing_page) = &self.value.data {
                    let click_event =
                        ClickEvent::create(ClickableElement::LandingPage(TerseElement::new(
                            landing_page.landing_page_id.clone(),
                            Some(landing_page.url.clone()),
                        )));
                    Ok((landing_page.url.clone(), click_event))
                } else {
                    Err(format!("matrix data not lander"))
                }
            }
        }
    }
    //

    pub fn select_children_recursively(m: LiveMatrix) -> Vec<ClickMap> {
        let mut nodes = vec![];

        for group in m.children_groups.iter() {
            let select = select_child(group);
            let value = select.value.clone();
            let children = Self::select_children_recursively(select);

            let map = ClickMap {
                children,
                value,
                seq_type: None,
                linked_conversion_id: None,
            };

            nodes.push(map);
        }

        nodes
    }

    pub fn generate_matrix(matrix: LiveMatrix) -> ClickMap {
        let matrix_group = matrix
            .children_groups
            .into_iter()
            .flatten()
            .collect::<Vec<_>>();
        let selected_matrix = select_child(&matrix_group);
        let value = selected_matrix.value.clone();
        let mut selected_children = ClickMap::select_children_recursively(selected_matrix);

        Self {
            children: selected_children,
            value,
            seq_type: Some(SequenceType::Matrix),
            linked_conversion_id: None,
        }
    }

    pub fn generate_landing_page(matrix: LiveMatrix) -> ClickMap {
        let value = matrix.value.clone();
        let selected_lander = select_child(&matrix.children_groups.get(0).expect("THDFHV"));
        let value = selected_lander.value;

        let selected_children = matrix
            .children_groups
            .iter()
            .enumerate()
            .filter(|(idx, i)| *idx != 0)
            .map(|(idx, s)| {
                let selected_child = select_child(s);
                let click_map = ClickMap {
                    children: vec![],
                    value: selected_child.value,
                    seq_type: None,
                    linked_conversion_id: None,
                };
                click_map
            })
            .collect::<Vec<_>>();

        Self {
            children: selected_children,
            value,
            seq_type: Some(SequenceType::LandingPages),
            linked_conversion_id: None,
        }
    }

    pub fn generate_offer(matrix: LiveMatrix) -> ClickMap {
        let group = &matrix.children_groups.get(0).expect("SDAFG");
        let selected = select_child(group);
        let value = selected.value;

        Self {
            children: vec![],
            value,
            seq_type: Some(SequenceType::Offers),
            linked_conversion_id: None,
        }
    }
}

// #[derive(Serialize, Deserialize, Clone, Debug)]
// pub enum ClickMap {
//     Offer(OfferClickMap),
//     LandingPage(LandingPageClickMap),
//     Matrix(ClickMap),
// }

impl ClickMap {
    pub fn from_campaign(
        campaign: &Campaign,
        geo_ip: &GeoIPData,
        ua_data: &UserAgentData,
        parameters: &HashMap<String, String>,
        referrer_url: Option<Url>,
    ) -> Self {
        match &campaign.campaign_core {
            Either::Left(funnel) => {
                if let Some(condi_seq) =
                    condi_qualify(&funnel.conditional_sequences, &geo_ip, &ua_data)
                {
                    Self::from_sequence(select_sequence(&condi_seq.sequences))
                } else {
                    Self::from_sequence(select_sequence(&funnel.default_sequences))
                }
            }

            Either::Right(sequence) => Self::from_sequence(sequence),
        }
    }

    pub fn from_sequence(seq: &Sequence) -> Self {
        let matrix = seq.matrix.read().unwrap();
        let live = LiveMatrix::from_matrix(&*matrix);

        match seq.sequence_type {
            SequenceType::Offers => ClickMap::generate_offer(live),
            SequenceType::LandingPages => ClickMap::generate_landing_page(live),
            SequenceType::Matrix => ClickMap::generate_matrix(live),
        }
    }
}

pub fn select_sequence(group: &Vec<Sequence>) -> &Sequence {
    let mut sw: SmoothWeight<usize> = SmoothWeight::new();
    for (idx, s) in group.iter().enumerate() {
        sw.add(idx, s.weight as isize);
    }

    group.get(sw.next().expect("GT%sdf")).expect("HTGfdg")
}

// impl ClickMap {
//     pub fn new(live_campaign_mirror: &LiveCampaign) -> Result<ClickMapResult, anyhow::Error> {
//         let mut sequence_id = None;
//
//         match &live_campaign_mirror.path {
//             Either::Left(a) => {
//                 let mut sw: SmoothWeight<&SlimSequenceMirror> = SmoothWeight::new();
//                 for i in a {
//                     sw.add(&i.sequence, i.weight as isize)
//                 }
//
//                 let select_sequence = &sw.next().unwrap();
//                 sequence_id = Some(select_sequence.sid.clone());
//
//                 match &select_sequence.path_info.0 {
//                     Either::Left(a) => {
//                         if a.landing_pages.is_empty() {
//                             // DL
//                             let mut sw: SmoothWeight<&OfferMirror> = SmoothWeight::new();
//
//                             for i in &a.offer_groups.get(0).expect("WEGRjuj").offers {
//                                 sw.add(&i.mirror, i.weight as isize)
//                             }
//
//                             let selected = sw.next().expect("G#szzf");
//
//                             let clickmap = ClickMap::DL(OfferClickMap {
//                                 offer_id: selected.oid,
//                                 offer_url: selected.url.clone(),
//                             });
//
//                             Ok(ClickMapResult {
//                                 click_map: clickmap,
//                                 sequence_id,
//                             })
//                         } else {
//                             let mut sw: SmoothWeight<&LandingPageMirror> = SmoothWeight::new();
//
//                             for i in a.landing_pages.iter() {
//                                 sw.add(&i.mirror, i.weight as isize)
//                             }
//
//                             let selected_landing_page = sw.next().expect("HG4wx");
//
//                             // Select Offer
//                             if a.offer_groups.len() == 1 {
//                                 let mut sw: SmoothWeight<&OfferMirror> = SmoothWeight::new();
//
//                                 for i in a.offer_groups.get(0).expect("G#$w").offers.iter() {
//                                     sw.add(&i.mirror, i.weight as isize)
//                                 }
//
//                                 let selected_offer = sw.next().expect("GF#szcc");
//
//                                 let offer_s = vec![OfferClickMap {
//                                     offer_id: selected_offer.oid,
//                                     offer_url: selected_offer.url.clone(),
//                                 }];
//
//                                 let click_map = ClickMap::LP(LandingPageClickMap {
//                                     landing_page_id: selected_landing_page.lpid,
//                                     landing_page_url: selected_landing_page.url.clone(),
//                                     offer_groups: offer_s,
//                                 });
//
//                                 Ok(ClickMapResult {
//                                     click_map,
//                                     sequence_id,
//                                 })
//                             } else {
//                                 // Many Offer Group
//                                 // LP
//                                 let mut sw: SmoothWeight<&LandingPageMirror> = SmoothWeight::new();
//
//                                 for i in a.landing_pages.iter() {
//                                     sw.add(&i.mirror, i.weight as isize)
//                                 }
//
//                                 let selected_landing_page = sw.next().expect("ejtj");
//
//                                 // Offer Groups
//                                 let mut offer_s: Vec<OfferClickMap> = vec![];
//
//                                 for group in a.offer_groups.iter() {
//                                     let mut sw: SmoothWeight<&OfferMirror> = SmoothWeight::new();
//
//                                     for i in group.offers.iter() {
//                                         sw.add(&i.mirror, i.weight as isize)
//                                     }
//
//                                     let selected_offer = sw.next().expect("G#s788");
//
//                                     offer_s.push(OfferClickMap {
//                                         offer_id: selected_offer.oid,
//                                         offer_url: selected_offer.url.clone(),
//                                     })
//                                 }
//
//                                 let click_map = ClickMap::LP(LandingPageClickMap {
//                                     landing_page_id: selected_landing_page.lpid,
//                                     landing_page_url: selected_landing_page.url.clone(),
//                                     offer_groups: offer_s,
//                                 });
//
//                                 Ok(ClickMapResult {
//                                     click_map,
//                                     sequence_id,
//                                 })
//                             }
//                         }
//                     }
//                     Either::Right(b) => {
//                         //MV
//                         // let mut sw:SmoothWeight<&MatrixPath>
//                         Err(anyhow::Error::msg("mv not setup"))
//                     }
//                 }
//                 // Ok()
//             }
//             Either::Right(b) => Err(anyhow::Error::msg("core not setup")),
//         }
//     }
// }
