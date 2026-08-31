// SPDX-License-Identifier: Apache-2.0

use anyhow::Result;
use oqtopus_cloud::apis::configuration::Configuration;
use oqtopus_cloud::apis::{device_api, job_api};

#[tokio::main]
async fn main() -> Result<()> {
    let configuration = build_configuration()?;

    // list_devices ----------------------------------------------------
    let devices = match device_api::list_devices(&configuration).await {
        Ok(devices) => devices,
        Err(e) => return Err(describe_api_error("list_devices", e)),
    };
    for device in &devices {
        println!(
            "device_id={}, type={:?}, status={:?}, n_qubits={:?}",
            device.device_id, device.device_type, device.status, device.n_qubits
        );
    }

    // list_jobs ---------------------------------------------------------
    match job_api::list_jobs(
        &configuration,
        Some("job_id,status,name"), // fields
        None,                       // start_time
        None,                       // end_time
        None,                       // status
        None,                       // q
        None,                       // page
        None,                       // size
        None,                       // order
    )
    .await
    {
        Ok(jobs) => println!("{:#?}", jobs),
        Err(e) => return Err(describe_api_error("list_jobs", e)),
    }

    // register_job_id ----------------------------------------------------
    match job_api::register_job_id(&configuration).await {
        Ok(register) => println!("{:#?}", register),
        Err(e) => return Err(describe_api_error("register_job_id", e)),
    }

    Ok(())
}

fn describe_api_error<T: std::fmt::Debug + Send + Sync + 'static>(
    context: &str,
    err: oqtopus_cloud::apis::Error<T>,
) -> anyhow::Error {
    match &err {
        oqtopus_cloud::apis::Error::ResponseError(rc) => {
            eprintln!(
                "[{context}] error status={}, content={}, entity={:?}",
                rc.status, rc.content, rc.entity
            );
        }
        other => eprintln!("[{context}] error: {other}"),
    }
    err.into()
}

fn build_configuration() -> Result<Configuration> {
    let token =
        std::env::var("OQTOPUS_API_TOKEN").expect("OQTOPUS_API_TOKEN environment variable not set");
    let token = token.trim();

    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        reqwest::header::HeaderName::from_static("q-api-token"),
        reqwest::header::HeaderValue::from_str(token)?,
    );
    headers.insert(
        reqwest::header::ACCEPT,
        reqwest::header::HeaderValue::from_static("application/json"),
    );

    let client = reqwest::Client::builder()
        .default_headers(headers)
        .build()?;

    let mut configuration = Configuration::new();
    configuration.base_path = std::env::var("OQTOPUS_BASE_URL")
        .unwrap_or_else(|_| "https://demo-api.oqtopus.io".to_string());
    configuration.client = client;
    configuration.user_agent = Some("oqtopus-client/1.1.6".to_string());

    Ok(configuration)
}
