use webdetect::crawler::WebCrawler;
use webdetect::scanner::{SqlInjectionScanner, XssScanner};
use webdetect::reporting::ReportingModule;
use webdetect::database::VulnerabilityDatabase;

#[tokio::test]
async fn test_integration() {
    // Set up modules
    let crawler = WebCrawler::new();
    let sql_injection_scanner = SqlInjectionScanner::new();
    let xss_scanner = XssScanner::new();
    let reporting_module = ReportingModule::new();
    let vulnerability_database = VulnerabilityDatabase::new();

    // Simulate crawling a test web page
    let test_url = "https://google.com/";
    let test_content = "Test web page content";
    let crawled_content = crawler.crawl(test_url).await.unwrap(); // Mock the crawl function or use a test HTTP server

    // Simulate scanning for vulnerabilities
    let sql_injection_results = sql_injection_scanner.scan_sql_injection(&crawled_content);
    let xss_results = xss_scanner.scan_xss(&crawled_content);

    // Combine results
    let mut all_results = vec![];
    all_results.extend(sql_injection_results);
    all_results.extend(xss_results);

    // Simulate storing vulnerabilities in the database
    for result in &all_results {
        vulnerability_database.add_vulnerability(result.clone());
    }

    // Simulate generating and displaying the report
    reporting_module.generate_report(all_results);

    // Simulate displaying all vulnerabilities stored in the database
    assert_eq!(
        vulnerability_database.get_vulnerabilities(),
        &all_results,
        "Stored vulnerabilities do not match expected results"
    );
}
