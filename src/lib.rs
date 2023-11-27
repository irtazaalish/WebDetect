// Import modules from subdirectories
mod crawler;
pub mod scanner;
mod reporting;
mod database;

// Export modules so they are accessible outside the library crate
pub use crawler::crawler::WebCrawler;
pub use crate::scanner::sql_injection::SqlInjectionScanner;
pub use crate::scanner::xss::XssScanner;
pub use reporting::reporting::ReportingModule;
pub use crate::database::VulnerabilityDatabase;
