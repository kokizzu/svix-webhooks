use super::PostOptions;
use crate::{apis::integration_api, error::Result, models::*, Configuration};

#[derive(Default)]
pub struct IntegrationListOptions {
    pub iterator: Option<String>,
    pub limit: Option<i32>,
    pub order: Option<Ordering>,
}

pub struct Integration<'a> {
    cfg: &'a Configuration,
}

impl<'a> Integration<'a> {
    pub(super) fn new(cfg: &'a Configuration) -> Self {
        Self { cfg }
    }

    pub async fn list(
        &self,
        app_id: String,
        options: Option<IntegrationListOptions>,
    ) -> Result<ListResponseIntegrationOut> {
        let IntegrationListOptions {
            iterator,
            limit,
            order,
        } = options.unwrap_or_default();
        integration_api::v1_period_integration_period_list(
            self.cfg,
            integration_api::V1PeriodIntegrationPeriodListParams {
                app_id,
                iterator,
                limit,
                order,
            },
        )
        .await
    }

    pub async fn create(
        &self,
        app_id: String,
        integration_in: IntegrationIn,
        options: Option<PostOptions>,
    ) -> Result<IntegrationOut> {
        let PostOptions { idempotency_key } = options.unwrap_or_default();
        integration_api::v1_period_integration_period_create(
            self.cfg,
            integration_api::V1PeriodIntegrationPeriodCreateParams {
                app_id,
                integration_in,
                idempotency_key,
            },
        )
        .await
    }

    pub async fn get(&self, app_id: String, integ_id: String) -> Result<IntegrationOut> {
        integration_api::v1_period_integration_period_get(
            self.cfg,
            integration_api::V1PeriodIntegrationPeriodGetParams { app_id, integ_id },
        )
        .await
    }

    pub async fn update(
        &self,
        app_id: String,
        integ_id: String,
        integration_update: IntegrationUpdate,
    ) -> Result<IntegrationOut> {
        integration_api::v1_period_integration_period_update(
            self.cfg,
            integration_api::V1PeriodIntegrationPeriodUpdateParams {
                app_id,
                integ_id,
                integration_update,
            },
        )
        .await
    }

    pub async fn delete(&self, app_id: String, integ_id: String) -> Result<()> {
        integration_api::v1_period_integration_period_delete(
            self.cfg,
            integration_api::V1PeriodIntegrationPeriodDeleteParams { app_id, integ_id },
        )
        .await
    }

    pub async fn get_key(&self, app_id: String, integ_id: String) -> Result<IntegrationKeyOut> {
        integration_api::v1_period_integration_period_get_key(
            self.cfg,
            integration_api::V1PeriodIntegrationPeriodGetKeyParams { app_id, integ_id },
        )
        .await
    }

    pub async fn rotate_key(&self, app_id: String, integ_id: String) -> Result<IntegrationKeyOut> {
        integration_api::v1_period_integration_period_rotate_key(
            self.cfg,
            integration_api::V1PeriodIntegrationPeriodRotateKeyParams {
                app_id,
                integ_id,
                idempotency_key: None,
            },
        )
        .await
    }
}
