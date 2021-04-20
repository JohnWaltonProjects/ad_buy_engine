use crate::data::elements::funnel::ConditionalSequence;
use crate::data::lists::condition::{Condition, ConditionDataType, ConnectionType};
use crate::data::visit::geo_ip::GeoIPData;
use crate::data::visit::user_agent::UserAgentData;

pub fn qualify_condition(
    geo_ip: &GeoIPData,
    ua_data: &UserAgentData,
    condition: &Condition,
) -> bool {
    match &condition.condition_data_type {
        ConditionDataType::ConnectionType(ct) => true,
        _ => false,
    }
}

fn find_match_connection_type(geo_ip: &GeoIPData, list_to_check: &Vec<ConnectionType>) -> bool {
    let user_conn = &geo_ip.connection_type;
    println!("{}", &user_conn);

    false
}

// IMPL FROM STR FOR CONNECTION TYPE

pub fn condi_qualify<'a>(
    condi_seqs: &'a Vec<ConditionalSequence>,
    geo_ip: &GeoIPData,
    ua_data: &UserAgentData,
) -> Option<&'a ConditionalSequence> {
    None
}

pub fn qualify(
    condi_seq: &ConditionalSequence,
    geo_ip: &GeoIPData,
    ua_data: &UserAgentData,
) -> bool {
    // let mut buff = vec![];
    if !condi_seq.conditional_sequence_is_active {
        return false;
    }

    for condition in condi_seq.condition_set.iter() {
        if !qualify_condition(&geo_ip, &ua_data, condition) {
            return false;
        }
    }

    true
}
