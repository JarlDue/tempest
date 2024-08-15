// src/main.rs
mod campaign;
mod engine;
mod reporter;

use campaign::Campaign;
use clap::Parser;
use engine::LoadEngine;
use engine::ScenarioResult;
use reporter::Reporter;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long, parse(from_os_str))]
    config: PathBuf,

    #[clap(long)]
    dry_run: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let yaml_str = std::fs::read_to_string(args.config)?;
    let campaign = Campaign::from_yaml(&yaml_str)?;

    let engine = LoadEngine::new(&campaign, args.dry_run);
    let results = engine.run().await?;

    println!("\nGenerating HTML report...");
    generate_html_report(&campaign, &results)?;

    Ok(())
}

use std::fs::File;
use std::io::Write;

fn generate_html_report(
    campaign: &Campaign,
    results: &[ScenarioResult],
) -> Result<(), Box<dyn std::error::Error>> {
    let mut html = String::new();

    // HTML head with embedded Tailwind CSS
    html.push_str(
        r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Tempest Load Test Report</title>
    <script src="https://cdn.tailwindcss.com"></script>
</head>
<body class="bg-gray-100 p-8">
    <div class="max-w-4xl mx-auto">
        <h1 class="text-3xl font-bold mb-6">Tempest Load Test Report</h1>
        <div class="bg-white shadow-md rounded px-8 pt-6 pb-8 mb-4">
            <h2 class="text-xl font-semibold mb-4">Campaign: "#,
    );
    html.push_str(&campaign.name);
    html.push_str(
        r#"</h2>
            <p class="mb-2"><strong>Version:</strong> "#,
    );
    html.push_str(&campaign.version);
    html.push_str(
        r#"</p>
            <p class="mb-2"><strong>Description:</strong> "#,
    );
    html.push_str(&campaign.description);
    html.push_str(
        r#"</p>
            <p class="mb-4"><strong>Base URL:</strong> "#,
    );
    html.push_str(&campaign.base_url);
    html.push_str(
        r#"</p>
            <h3 class="text-lg font-semibold mb-2">Success Criteria:</h3>
            <ul class="list-disc list-inside mb-4">
                <li>Max Response Time: "#,
    );
    html.push_str(&campaign.success_criteria.max_response_time.to_string());
    html.push_str(
        r#" ms</li>
                <li>Error Rate Threshold: "#,
    );
    html.push_str(&format!(
        "{:.2}%",
        campaign.success_criteria.error_rate_threshold * 100.0
    ));
    html.push_str(
        r#"</li>
            </ul>
        </div>
        <h2 class="text-2xl font-bold mb-4">Scenario Results</h2>
    "#,
    );

    for result in results {
        let error_rate = result.failed_requests as f32 / result.requests_sent as f32 * 100.0;
        let error_rate_class =
            if error_rate > campaign.success_criteria.error_rate_threshold * 100.0 {
                "text-red-600"
            } else {
                "text-green-600"
            };
        let max_response_time_class =
            if result.max_response_time > campaign.success_criteria.max_response_time {
                "text-red-600"
            } else {
                "text-green-600"
            };

        html.push_str(&format!(r#"
        <div class="bg-white shadow-md rounded px-8 pt-6 pb-8 mb-4">
            <h3 class="text-xl font-semibold mb-4">{}</h3>
            <div class="grid grid-cols-2 gap-4">
                <div>
                    <p class="mb-2"><strong>Requests Sent:</strong> {}</p>
                    <p class="mb-2"><strong>Successful Requests:</strong> {}</p>
                    <p class="mb-2"><strong>Failed Requests:</strong> {}</p>
                    <p class="mb-2"><strong>Error Rate:</strong> <span class="{}">{:.2}%</span></p>
                </div>
                <div>
                    <p class="mb-2"><strong>Avg Response Time:</strong> {:.2} ms</p>
                    <p class="mb-2"><strong>Min Response Time:</strong> {} ms</p>
                    <p class="mb-2"><strong>Max Response Time:</strong> <span class="{}">{} ms</span></p>
                </div>
            </div>
        </div>
    "#,
        result.name, result.requests_sent, result.successful_requests, result.failed_requests,
        error_rate_class, error_rate,
        result.avg_response_time, result.min_response_time,
        max_response_time_class, result.max_response_time
    ));
    }

    html.push_str(
        r#"
    </div>
</body>
</html>
    "#,
    );

    let mut file = File::create("report.html")?;
    file.write_all(html.as_bytes())?;

    println!("Report generated: report.html");

    Ok(())
}
