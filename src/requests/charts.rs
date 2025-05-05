use serde_derive::{Deserialize, Serialize};

use crate::{responses::charts::{ActiveUserChart, ApRequestChart, DriveChart, FederationChart}, FixedEndpointMisskeyClientRequest};

#[derive(Clone, Copy, Debug, Serialize)]
pub struct CommonChartRequestBody {
    span: ChartSpan,
    limit: usize,
    offset: Option<usize>,
}

impl CommonChartRequestBody {
    pub fn new(span: ChartSpan) -> Self {
        Self { span, limit: 30, offset: None }
    }

    pub fn limit(self, limit: usize) -> Self {
        Self { limit, .. self }
    }

    pub fn offset(self, offset: usize) -> Self {
        Self { offset: Some(offset), .. self }
    }
}

#[derive(Debug, Serialize)]
pub struct GetActiveUsersChart {
    #[serde(flatten)]
    common_body: CommonChartRequestBody,
}

impl GetActiveUsersChart {
    pub fn new(common_body: CommonChartRequestBody) -> Self {
        Self { common_body }
    }
}

impl FixedEndpointMisskeyClientRequest for GetActiveUsersChart {
    const ENDPOINT: &'static str = "/charts/active-users";

    type Response = ActiveUserChart;
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ChartSpan {
    Day,
    Hour,
}

#[derive(Debug, Serialize)]
pub struct GetApRequestChart {
    #[serde(flatten)]
    common_body: CommonChartRequestBody,
}

impl GetApRequestChart {
    pub fn new(common_body: CommonChartRequestBody) -> Self {
        Self { common_body }
    }
}

impl FixedEndpointMisskeyClientRequest for GetApRequestChart {
    const ENDPOINT: &'static str = "/charts/ap-request";

    type Response = ApRequestChart;
}

#[derive(Debug, Serialize)]
pub struct GetDriveChart {
    #[serde(flatten)]
    common_body: CommonChartRequestBody,
}

impl GetDriveChart {
    pub fn new(common_body: CommonChartRequestBody) -> Self {
        Self { common_body }
    }
}

impl FixedEndpointMisskeyClientRequest for GetDriveChart {
    const ENDPOINT: &'static str = "/charts/drive";

    type Response = DriveChart;
}

#[derive(Debug, Serialize)]
pub struct GetFederationChart {
    common_body: CommonChartRequestBody,
}

impl GetFederationChart {
    pub fn new(common_body: CommonChartRequestBody) -> Self {
        Self { common_body }
    }
}

impl FixedEndpointMisskeyClientRequest for GetFederationChart {
    const ENDPOINT: &'static str = "/charts/federation";

    type Response = FederationChart;
}
