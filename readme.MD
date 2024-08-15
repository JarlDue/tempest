# Tempest

Tempest is a flexible, declarative load testing tool built in Rust. It allows you to define load test scenarios in YAML and generates detailed HTML reports of test results.

## Features

- Declarative test scenarios defined in YAML
- Simulates various HTTP methods (GET, POST, PUT, DELETE, PATCH)
- Supports custom headers, query parameters, and request bodies
- Generates detailed HTML reports with Tailwind CSS for easy analysis
- Customizable success criteria for response times and error rates

## Installation

To install Tempest, you need to have Rust and Cargo installed on your system. If you don't have them installed, you can get them from [https://rustup.rs/](https://rustup.rs/).

Once you have Rust and Cargo, you can install Tempest by cloning this repository and building it:

```bash
git clone https://github.com/yourusername/tempest.git
cd tempest
cargo build --release
```

The built binary will be in `target/release/tempest`.

## Usage

1. Create a YAML file defining your load test campaign. Here's a basic example:

```yaml
name: "E-commerce Platform Load Test Campaign"
version: "1.0"
description: "Basic load test campaign for our e-commerce platform"
base_url: "https://api.example-ecommerce.com"

scenarios:
  - name: "Homepage Browsing"
    endpoint: "/home"
    method: "GET"
    rate: 50  # requests per second
    duration: 300  # seconds

  - name: "Product Search"
    endpoint: "/search"
    method: "GET"
    rate: 30
    duration: 180
    query_params:
      q: "smartphone"

success_criteria:
  max_response_time: 500  # milliseconds
  error_rate_threshold: 0.01  # 1%
```

2. Run Tempest with your campaign file:

```bash
./tempest path/to/your/campaign.yaml
```

3. After the test completes, Tempest will generate an HTML report (`report.html`) in the current directory. Open this file in a web browser to view the detailed results.

## Contributing

Contributions to Tempest are welcome! Please feel free to submit a Pull Request.

## Acknowledgments

- This project uses [serde](https://serde.rs/) for YAML parsing.
- The HTML report is styled with [Tailwind CSS](https://tailwindcss.com/).
