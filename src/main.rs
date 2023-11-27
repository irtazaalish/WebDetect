use std::env;
use crawler::WebCrawler;
use web_detect::scanner::sql_injection::SqlInjectionScanner;
use web_detect::scanner::xss::XssScanner;
use reporting::ReportingModule;
use database::VulnerabilityDatabase;

/// The main entry point for the web application scanner.
///
/// This program crawls a specified web page, scans for common vulnerabilities,
/// generates a report, and stores vulnerabilities in a database.
#[tokio::main]
async fn main() {
    // Read command line arguments
    let args: Vec<String> = env::args().collect();

    // Check if a URL argument is provided
    if args.len() < 2 {
        eprintln!("Usage: {} <target_url>", args[0]);
        return;
    }

    // Extract the target URL from the command line arguments
    let url = &args[1];

    // Initialize modules
    let crawler = WebCrawler::new();
    let sql_injection_scanner = SqlInjectionScanner::new();
    let xss_scanner = XssScanner::new();
    let reporting_module = ReportingModule::new();
    let mut vulnerability_database = VulnerabilityDatabase::new(); // Declare as mutable

    // Crawl the web and handle errors
    let content = match crawler.crawl(url).await {
        Ok(content) => content,
        Err(err) => {
            eprintln!("Error while crawling: {:?}", err);
            return;
        }
    };

    // Scan for SQL injection vulnerabilities
    let sql_injection_results = sql_injection_scanner.scan_sql_injection(&content);

    // Scan for XSS vulnerabilities
    let xss_results = xss_scanner.scan_xss(&content);

    // Combine results
    let mut all_results: Vec<String> = vec![];

    all_results.extend(sql_injection_results);
    all_results.extend(xss_results);

    // Store vulnerabilities in the database
    for result in &all_results {
        vulnerability_database.add_vulnerability(result.clone());
    }

    // Generate and display the report
    reporting_module.generate_report(all_results);

    // Display all vulnerabilities stored in the database
    println!("Vulnerabilities stored in the database: {:?}", vulnerability_database.get_vulnerabilities());
}
