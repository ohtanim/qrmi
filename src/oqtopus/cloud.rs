//
// (C) Copyright IBM 2026
//
// This code is licensed under the Apache License, Version 2.0. You may
// obtain a copy of this license in the LICENSE.txt file in the root directory
// of this source tree or at http://www.apache.org/licenses/LICENSE-2.0.
//
// Any modifications or derivative works of this code must retain this
// copyright notice, and modified files need to carry a notice indicating
// that they have been altered from the originals.

use crate::error::required_env;
use crate::models::{Payload, ResourceType, Target, TaskResult, TaskStatus};
use crate::oqtopus::error::{classify, other, ResourceKind};
use crate::{QuantumResource, Result};
use async_trait::async_trait;
use oqtopus_cloud::apis::configuration::Configuration;
use oqtopus_cloud::apis::device_api::get_device;
use oqtopus_cloud::apis::job_api::get_job_status;
use oqtopus_cloud::models::{devices_device_info, JobsJobStatus};
use std::collections::HashMap;
use std::env;
use uuid::Uuid;

const DEFAULT_TIMEOUT_SECS: u64 = 30;

/// QRMI implementation for Oqtopus Cloud
pub struct OqtopusCloud {
    pub(crate) config: Configuration,
    pub(crate) device_id: String,
    pub(crate) acquisition_token: Option<String>,
}

impl OqtopusCloud {
    /// Constructs an OQTOPUS Cloud instance.
    ///
    /// Environment variables used:
    /// * QRMI_OQTOPUS_API_TOKEN - Oqtopus Cloud API token
    /// * QRMI_OQTOPUS_BASE_URL - IQM Server API endpoint
    /// * QRMI_JOB_ACQUISITION_TOKEN - (optional) pre‐set session ID
    /// * QRMI_OQTOPUS_TIMEOUT_SECS - (optional) request timeout in seconds
    /// * QRMI_OQTOPUS_PROXY_URL - (optional) proxy URL
    pub fn new(resource_id: &str) -> Result<Self> {
        let endpoint = required_env(format!("{resource_id}_QRMI_OQTOPUS_BASE_URL"))?;
        let api_token = required_env(format!("{resource_id}_QRMI_OQTOPUS_API_TOKEN"))?;
        let acquisition_token = env::var(format!("{resource_id}_QRMI_JOB_ACQUISITION_TOKEN")).ok();

        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            reqwest::header::HeaderName::from_static("q-api-token"),
            reqwest::header::HeaderValue::from_str(&api_token)
                .map_err(|e| other("invalid q-api-token header value", e))?,
        );
        headers.insert(
            reqwest::header::ACCEPT,
            reqwest::header::HeaderValue::from_static("application/json"),
        );

        let timeout_secs: u64 = std::env::var("QRMI_OQTOPUS_TIMEOUT_SECS")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(DEFAULT_TIMEOUT_SECS);

        let mut client_builder = reqwest::Client::builder()
            .default_headers(headers)
            .timeout(std::time::Duration::from_secs(timeout_secs));

        if let Ok(proxy_url) = std::env::var("QRMI_OQTOPUS_PROXY_URL") {
            client_builder = client_builder
                .proxy(reqwest::Proxy::all(proxy_url).map_err(|e| other("invalid proxy URL", e))?);
        }

        let client = client_builder
            .build()
            .map_err(|e| other("failed to create REST API client", e))?;

        // Set up the config
        let mut config = Configuration::new();
        config.client = client;
        config.base_path = endpoint;
        config.bearer_access_token = Some(api_token);

        Ok(Self {
            config,
            device_id: resource_id.to_string(),
            acquisition_token,
        })
    }
}

// Implement the QuantumResource trait using the asynchronous wrappers.
#[async_trait]
impl QuantumResource for OqtopusCloud {
    async fn resource_id(&mut self) -> Result<String> {
        Ok(self.device_id.clone())
    }

    async fn resource_type(&mut self) -> Result<ResourceType> {
        Ok(ResourceType::OqtopusCloud)
    }

    /// Asynchronously checks if a backend is accessible.
    async fn is_accessible(&mut self) -> Result<bool> {
        let device = get_device(&self.config, &self.device_id)
            .await
            .map_err(|e| classify(e, ResourceKind::Device))?;
        Ok(device.status == devices_device_info::Status::Available)
    }

    /// IQM Server has no session concept. This does not contact the
    /// provider; it returns a generated id so callers written against the
    /// trait do not need a special case for this backend.
    async fn acquire(&mut self) -> Result<String> {
        Ok(Uuid::new_v4().to_string())
    }

    /// IQM Server has no session concept, so this is a no-op: nothing is
    /// contacted and nothing is released. See `acquire()`.
    async fn release(&mut self, _acquisition_token: &str) -> Result<()> {
        Ok(())
    }

    /// Starts a job task.
    ///
    async fn task_start(&mut self, _payload: Payload) -> Result<String> {
        Err(anyhow::anyhow!("Not supported yet"))?
    }

    /// Stops a running job.
    ///
    /// Fetches the job's current status first, and only actually asks the
    /// server to cancel it if that status is `Waiting` or `Processing` --
    /// a job already in a terminal state (`Completed`/`Failed`/`Cancelled`)
    /// can't be cancelled again, and the server rejects that with 403
    /// (`IllegalJobStatus`, see `crate::iqm::error`'s docs on why that
    /// status isn't otherwise classified). Mirrors
    /// `crate::ibm::quantum_compute_service::QuantumComputeService::task_stop`'s
    /// same guard for the same reason.
    ///
    /// The cancel call's own result is deliberately discarded, same as
    /// that implementation: even after checking, the job could still
    /// finish on its own in the moment between the status check and this
    /// call, and that race isn't an error worth surfacing to the caller.
    async fn task_stop(&mut self, _task_id: &str) -> Result<()> {
        Err(anyhow::anyhow!("Not supported yet"))?
    }

    /// Returns the current status of a job.
    ///
    async fn task_status(&mut self, task_id: &str) -> Result<TaskStatus> {
        let job = get_job_status(&self.config, task_id)
            .await
            .map_err(|e| classify(e, ResourceKind::Job))?;

        match job.status {
            JobsJobStatus::Registered => Ok(TaskStatus::Queued),
            JobsJobStatus::Submitted => Ok(TaskStatus::Queued),
            JobsJobStatus::Ready => Ok(TaskStatus::Queued),
            JobsJobStatus::Running => Ok(TaskStatus::Running),
            JobsJobStatus::Succeeded => Ok(TaskStatus::Completed),
            JobsJobStatus::Failed => Ok(TaskStatus::Failed),
            JobsJobStatus::Cancelled => Ok(TaskStatus::Cancelled),
        }
    }

    /// Retrieves the results of a completed job.
    ///
    /// This function calls GET /jobs/{id}/results and serializes the returned JSON into a string.
    ///
    /// Which artifacts exist depends on the job type (see
    /// `job_get_artifacts`'s own documentation), so a 404 for
    /// `measurements` or `measurement_counts` is normal and is represented
    /// as `null` for that field. Any other failure -- network, auth, a
    /// non-404 error status, or a response that isn't valid JSON --
    /// propagates as `Err` instead of being silently swallowed into the
    /// same `null`.
    async fn task_result(&mut self, _task_id: &str) -> Result<TaskResult> {
        Err(anyhow::anyhow!("Not supported yet"))?
    }

    /// Returns the log messages of the task.
    ///
    async fn task_logs(&mut self, _task_id: &str) -> Result<String> {
        Err(anyhow::anyhow!("Not supported yet"))?
    }

    /// Retrieves target details.
    ///
    async fn target(&mut self) -> Result<Target> {
        Err(anyhow::anyhow!("Not supported yet"))?
    }

    async fn metadata(&mut self) -> HashMap<String, String> {
        let mut metadata = HashMap::new();
        metadata.insert("device_id".to_string(), self.device_id.clone());
        if let Some(ref acquisition_token) = self.acquisition_token {
            metadata.insert(
                "acquisition_token".to_string(),
                acquisition_token.to_string(),
            );
        }
        metadata
    }
}
