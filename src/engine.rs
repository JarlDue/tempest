use crate::campaign::*;
use rand::Rng;
use std::time::Duration;

pub struct LoadEngine<'a> {
    campaign: &'a Campaign,
}

impl<'a> LoadEngine<'a> {
    pub fn new(campaign: &'a Campaign) -> Self {
        LoadEngine { campaign }
    }

    pub fn run(&self) -> Result<Vec<ScenarioResult>, Box<dyn std::error::Error>> {
        println!("Starting campaign: {}", self.campaign.name);
        println!("Base URL: {}", self.campaign.base_url);

        let mut results = Vec::new();

        for scenario in &self.campaign.scenarios {
            let result = self.run_scenario(scenario)?;
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

    fn run_scenario(
        &self,
        scenario: &Scenario,
    ) -> Result<ScenarioResult, Box<dyn std::error::Error>> {
        println!("\nExecuting scenario: {}", scenario.name);
        println!("Endpoint: {}", scenario.endpoint);
        println!("Method: {:?}", scenario.method);
        println!("Rate: {} requests per second", scenario.rate);
        println!("Duration: {} seconds", scenario.duration);

        if !scenario.query_params.is_empty() {
            println!("Query params: {:?}", scenario.query_params);
        }

        if let Some(json) = &scenario.json_content {
            println!("JSON content: {}", serde_json::to_string_pretty(json)?);
        }

        if let Some(raw) = &scenario.raw_content {
            println!("Raw content: {}", raw);
        }

        // Simulate running the scenario and generate mock results
        println!(
            "Simulating requests for {} seconds at {} RPS",
            scenario.duration, scenario.rate
        );
        std::thread::sleep(Duration::from_secs(1)); // Simulate some work

        let result = self.generate_mock_result(scenario);
        println!("Scenario '{}' completed.", scenario.name);
        println!("Result: {:?}", result);

        Ok(result)
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
    pub avg_response_time: f32,
    pub max_response_time: u32,
    pub min_response_time: u32,
}
