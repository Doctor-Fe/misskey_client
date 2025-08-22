use derive_getters::Getters;
use serde_derive::Deserialize;

#[derive(Debug, Deserialize, Getters)]
#[serde(rename_all = "camelCase")]
pub struct ActiveUserChart {
    read_write: Vec<usize>,
    read: Vec<usize>,
    write: Vec<usize>,
    registered_within_week: Vec<usize>,
    registered_within_month: Vec<usize>,
    registered_within_year: Vec<usize>,
    registered_outside_week: Vec<usize>,
    registered_outside_month: Vec<usize>,
    registered_outside_year: Vec<usize>,
}

#[derive(Debug, Deserialize, Getters)]
#[serde(rename_all = "camelCase")]
pub struct ApRequestChart {
    deliver_failed: Vec<usize>,
    deliver_succeeded: Vec<usize>,
    inbox_received: Vec<usize>
}

#[derive(Debug, Deserialize, Getters)]
#[serde(rename_all = "camelCase")]
pub struct DriveChart {
    local: DriveChartDetail,
    remote: DriveChartDetail,
}

#[derive(Debug, Deserialize, Getters)]
#[serde(rename_all = "camelCase")]
pub struct DriveChartDetail {
    inc_count: Vec<usize>,
    inc_size: Vec<usize>,
    dec_count: Vec<usize>,
    dec_size: Vec<usize>,
}

#[derive(Debug, Deserialize, Getters)]
#[serde(rename_all = "camelCase")]
pub struct FederationChart {
    delivered_instances: Vec<usize>,
    inbox_instances: Vec<usize>,
    stalled: Vec<usize>,
    sub: Vec<usize>,
    #[serde(rename = "pub")]
    #[getter(rename = "get_pub")]
    public: Vec<usize>,
    pubsub: Vec<usize>,
    sub_active: Vec<usize>,
    pub_active: Vec<usize>,
}
