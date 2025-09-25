use std::fmt::Debug;

use misskey_client_macroes::ConstParamJsonRequest;
use serde_derive::Serialize;

use crate::{common::ChartSpan, responses::charts::{ActiveUserChart, ApRequestChart, DriveChart, FederationChart}};

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

#[derive(Debug, Serialize, ConstParamJsonRequest)]
#[misskey_client(endpoint = "/chars/active-users", response = ActiveUserChart)]
pub struct GetActiveUsersChart {
    #[serde(flatten)]
    common_body: CommonChartRequestBody,
}

impl GetActiveUsersChart {
    pub fn new(common_body: CommonChartRequestBody) -> Self {
        Self { common_body }
    }
}

#[derive(Debug, Serialize, ConstParamJsonRequest)]
#[misskey_client(endpoint = "/charts/ap-request", response = ApRequestChart)]
pub struct GetApRequestChart {
    #[serde(flatten)]
    common_body: CommonChartRequestBody,
}

impl GetApRequestChart {
    pub fn new(common_body: CommonChartRequestBody) -> Self {
        Self { common_body }
    }
}

#[derive(Debug, Serialize, ConstParamJsonRequest)]
#[misskey_client(endpoint = "/charts/drive", response = DriveChart)]
pub struct GetDriveChart {
    #[serde(flatten)]
    common_body: CommonChartRequestBody,
}

impl GetDriveChart {
    pub fn new(common_body: CommonChartRequestBody) -> Self {
        Self { common_body }
    }
}

#[derive(Debug, Serialize, ConstParamJsonRequest)]
#[misskey_client(endpoint = "/charts/federation", response = FederationChart)]
pub struct GetFederationChart {
    common_body: CommonChartRequestBody,
}

impl GetFederationChart {
    pub fn new(common_body: CommonChartRequestBody) -> Self {
        Self { common_body }
    }
}
