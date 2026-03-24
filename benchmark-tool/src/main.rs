use std::process::Command;
use std::time::{Duration, Instant};
use std::thread;

const SERVICES: &[(&str, u16)] = &[
    ("go-gin", 8081),
    ("node-express", 8082),
    ("python-fastapi", 8083),
    ("java-spring", 8084),
    ("php-laravel", 8085),
];

fn main() {
    let mut context_path = String::from(".");

    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--path" | "-p" => {
                if let Some(path) = args.next() {
                    context_path = path;
                } else {
                    eprintln!("Error: --path requires a value");
                    std::process::exit(1);
                }
            }
            _ => {
                eprintln!("Unknown argument: {}", arg);
                std::process::exit(1);
            }
        }
    }

    let docker_compose_file = format!("{}/docker-compose.yml", context_path);

    println!("--- Starting Benchmark Build ---");
    let build_status = Command::new("docker")
        .args(&["compose", "-f", &docker_compose_file, "build"])
        .status()
        .expect("Failed to build images");

    if !build_status.success() {
        eprintln!("Docker build failed");
        return;
    }

    let mut results = Vec::new();

    for &(name, port) in SERVICES {
        println!("\n--- Benchmarking {} (Port {}) ---", name, port);
        
        // Start service
        Command::new("docker")
            .args(&["compose", "-f", &docker_compose_file, "up", "-d", name])
            .status()
            .expect("Failed to start service");

        // Wait for service to warm up
        thread::sleep(Duration::from_secs(10));

        // Get Image Size
        let img_name = format!("todo-api-bench-{}", name);
        let size_out = Command::new("docker")
            .args(&["images", &img_name, "--format", "{{.Size}}"])
            .output()
            .expect("Failed to get image size");
        let size = String::from_utf8_lossy(&size_out.stdout).trim().to_string();

        // Warm up request
        let _ = Command::new("curl")
            .args(&["-X", "POST", "-H", "Content-Type: application/json", "-d", "{\"title\":\"Warmup\",\"completed\":false}", &format!("http://localhost:{}/todos", port)])
            .output();

        // Performance Test
        let start = Instant::now();
        let requests = 500;
        for _ in 0..requests {
            let _ = Command::new("curl")
                .args(&["-s", &format!("http://localhost:{}/todos", port)])
                .output();
        }
        let duration = start.elapsed().as_secs_f64();
        let rps = requests as f64 / duration;

        // Resource Stats
        let stats_out = Command::new("docker")
            .args(&["stats", "--no-stream", "--format", "{{.CPUPerc}}|{{.MemUsage}}", &format!("todo-api-bench-{}-1", name)])
            .output()
            .expect("Failed to get stats");
        let stats_str = String::from_utf8_lossy(&stats_out.stdout).trim().to_string();
        let stats: Vec<&str> = stats_str.split('|').collect();
        let cpu = stats.get(0).unwrap_or(&"N/A").to_string();
        let mem = stats.get(1).unwrap_or(&"N/A").split('/').next().unwrap_or("N/A").trim().to_string();

        results.push((name, rps, cpu, mem, size));

        // Stop service
        Command::new("docker")
            .args(&["compose", "-f", &docker_compose_file, "stop", name])
            .status()
            .expect("Failed to stop service");
    }

    println!("\n\n{:<15} | {:<10} | {:<10} | {:<15} | {:<10}", "Service", "RPS", "CPU %", "Memory", "Img Size");
    println!("{}", "-".repeat(70));
    for (name, rps, cpu, mem, size) in &results {
        println!("{:<15} | {:<10.2} | {:<10} | {:<15} | {:<10}", name, rps, cpu, mem, size);
    }

    // Write to CSV
    let reports_dir = format!("{}/reports", context_path);
    std::fs::create_dir_all(&reports_dir).expect("Failed to create reports directory");
    let timestamp_output = Command::new("date")
        .args(&["+%Y%m%d_%H%M%S"])
        .output()
        .expect("Failed to get date");
    let timestamp = String::from_utf8_lossy(&timestamp_output.stdout).trim().to_string();
    
    let csv_file_path = format!("{}/benchmark_{}.csv", reports_dir, timestamp);
    let mut csv_content = String::from("Service,RPS,CPU %,Memory,Img Size\n");
    for (name, rps, cpu, mem, size) in results {
        csv_content.push_str(&format!("{},{:.2},{},{},{}\n", name, rps, cpu, mem, size));
    }
    std::fs::write(&csv_file_path, csv_content).expect("Failed to write CSV file");
    println!("\nResults saved to {}", csv_file_path);
}
