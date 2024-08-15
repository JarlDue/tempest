use crate::campaign::{Campaign, HttpMethod, Scenario};
use rand::Rng;
use regex::Regex;
use reqwest;
use std::collections::HashMap;

use reqwest::Client;
use std::time::Instant;
use tokio;

pub struct LoadEngine<'a> {
    campaign: &'a Campaign,
    dry_run: bool,
    client: reqwest::Client,
}

impl<'a> LoadEngine<'a> {
    pub fn new(campaign: &'a Campaign, dry_run: bool) -> Self {
        LoadEngine {
            campaign,
            dry_run,
            client: reqwest::Client::new(),
        }
    }

    pub async fn run(&self) -> Result<Vec<ScenarioResult>, Box<dyn std::error::Error>> {
        println!("Starting campaign: {}", self.campaign.name);
        println!("Base URL: {}", self.campaign.base_url);

        let mut results = Vec::new();

        for scenario in &self.campaign.scenarios {
            let result = self.run_scenario(scenario).await?;
            results.push(result);
        }

        println!("Campaign completed. Evaluating success criteria:");
        println!(
            "Max response time: {} ms",
            self.campaign.success_criteria.max_response_time
        );
        println!(
            "Error rate threshold: {:.2}%",
            self.campaign.success_criteria.error_rate_threshold * 100.0
        );

        Ok(results)
    }

    fn extract_from_response(
        &self,
        scenario: &Scenario,
        body: &str,
    ) -> Option<HashMap<String, String>> {
        scenario.response.as_ref().map(|response| {
            response
                .extract
                .iter()
                .filter_map(|(key, regex)| {
                    Regex::new(regex)
                        .ok()
                        .and_then(|re| re.captures(body))
                        .and_then(|caps| caps.get(1))
                        .map(|m| (key.clone(), m.as_str().to_string()))
                })
                .collect()
        })
    }

    async fn run_scenario(
        &self,
        scenario: &Scenario,
    ) -> Result<ScenarioResult, Box<dyn std::error::Error>> {
        println!("\nExecuting scenario: {}", scenario.name);
        println!("Endpoint: {}", scenario.endpoint);
        println!("Method: {:?}", scenario.method);
        println!("Rate: {} requests per second", scenario.rate);
        println!("Duration: {} seconds", scenario.duration);

        if self.dry_run {
            println!("Dry run mode: simulating requests");
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
            Ok(self.generate_mock_result(scenario))
        } else {
            self.execute_scenario(scenario).await
        }
    }

    async fn execute_scenario(
        &self,
        scenario: &Scenario,
    ) -> Result<ScenarioResult, Box<dyn std::error::Error>> {
        let start_time = std::time::Instant::now();
        let mut successful_requests = 0;
        let mut failed_requests = 0;
        let mut total_response_time = 0.0;
        let mut max_response_time = 0;
        let mut min_response_time = u32::MAX;

        let total_requests = scenario.rate * scenario.duration;
        let delay = tokio::time::Duration::from_secs_f64(1.0 / scenario.rate as f64);

        for _ in 0..total_requests {
            let request_start = std::time::Instant::now();
            match self.send_request(scenario).await {
                Ok((response_time, extracted)) => {
                    successful_requests += 1;
                    total_response_time += response_time as f64;
                    max_response_time = max_response_time.max(response_time);
                    min_response_time = min_response_time.min(response_time);

                    if let Some(extracted) = extracted {
                        println!("Extracted data: {:?}", extracted);
                    }
                }
                Err(_) => failed_requests += 1,
            }
            tokio::time::sleep_until(tokio::time::Instant::now() + delay).await;
        }

        Ok(ScenarioResult {
            name: scenario.name.clone(),
            requests_sent: total_requests,
            successful_requests,
            failed_requests,
            avg_response_time: total_response_time / successful_requests as f64,
            max_response_time,
            min_response_time,
        })
    }

    async fn send_request(
        &self,
        scenario: &Scenario,
    ) -> Result<(u32, Option<HashMap<String, String>>), Box<dyn std::error::Error>> {
        let url = format!("{}{}", self.campaign.base_url, scenario.endpoint);

        let start_time = Instant::now();
        let response = match scenario.method {
            HttpMethod::GET => self.client.get(&url).send().await?,
            HttpMethod::POST => {
                if let Some(json) = &scenario.json_content {
                    self.client.post(&url).json(json).send().await?
                } else {
                    self.client.post(&url).send().await?
                }
            }
            // Add other HTTP methods as needed
            _ => return Err("Unsupported HTTP method".into()),
        };

        let elapsed = start_time.elapsed();
        let status = response.status();

        if status.is_success() {
            let body = response.text().await?;
            let extracted = self.extract_from_response(scenario, &body);
            Ok((elapsed.as_millis() as u32, extracted))
        } else {
            Err(format!("Request failed with status: {}", status).into())
        }
    }

    fn generate_mock_result(&self, scenario: &Scenario) -> ScenarioResult {
        let mut rng = rand::thread_rng();
        let total_requests = scenario.rate * scenario.duration;
        let failed_requests = (total_requests as f32 * rng.gen_range(0.001..0.02)) as u32;

        ScenarioResult {
            name: scenario.name.clone(),
            requests_sent: total_requests,
            successful_requests: total_requests - failed_requests,
            failed_requests,
            avg_response_time: rng.gen_range(50.0..300.0),
            max_response_time: rng.gen_range(200..500),
            min_response_time: rng.gen_range(10..50),
        }
    }
}

#[derive(Debug)]
pub struct ScenarioResult {
    pub name: String,
    pub requests_sent: u32,
    pub successful_requests: u32,
    pub failed_requests: u32,
    pub avg_response_time: f64,
    pub max_response_time: u32,
    pub min_response_time: u32,
}
