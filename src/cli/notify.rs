use crate::notify::Notifier;
use crate::summarize::Report;
use anyhow::{Context, Result};
use std::path::PathBuf;

/// Runs the notify subcommand: loads the generated report, dispatches via Notifier backends,
/// and records delivery results.
pub async fn run() -> Result<()> {
    // Load the latest report
    let report = load_latest_report().context("Failed to load report. Run 'summarize' first.")?;

    // TODO: Load config and instantiate configured notifiers
    // For now, this is a placeholder
    // let config = Config::load()?;
    // 
    // let mut notifiers: Vec<Box<dyn Notifier>> = Vec::new();
    // if config.notifications.email.enabled {
    //     notifiers.push(Box::new(notify::email::EmailNotifier::new(&config)?));
    // }
    // if config.notifications.shortcut.enabled {
    //     notifiers.push(Box::new(notify::shortcut::ShortcutNotifier::new(&config)?));
    // }
    //
    // for notifier in notifiers {
    //     match notifier.send(&report).await {
    //         Ok(()) => println!("Notification sent successfully"),
    //         Err(e) => eprintln!("Notification failed: {}", e),
    //     }
    // }

    println!("Notification delivery completed (placeholder)");
    Ok(())
}

fn load_latest_report() -> Result<Report> {
    let report_path = get_latest_report_path()?;
    let markdown = std::fs::read_to_string(&report_path)?;
    
    // Extract date from filename
    let date = report_path
        .file_stem()
        .and_then(|s| s.to_str())
        .ok_or_else(|| anyhow::anyhow!("Invalid report filename"))?
        .to_string();

    // For now, create a minimal Report
    // In a full implementation, we'd parse the markdown or store metadata separately
    use crate::summarize::models::ReportMetadata;
    use chrono::Local;
    
    let metadata = ReportMetadata::new(Local::now().to_rfc3339());
    
    Ok(Report {
        date,
        markdown,
        metadata,
    })
}

fn get_latest_report_path() -> Result<PathBuf> {
    let home = std::env::var("HOME")?;
    let reports_dir = PathBuf::from(format!("{}/.local/share/news_agent/reports", home));
    
    // Find the most recent report file
    let mut reports: Vec<PathBuf> = std::fs::read_dir(&reports_dir)?
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .filter(|path| path.extension().map(|e| e == "md").unwrap_or(false))
        .collect();
    
    reports.sort();
    reports.last()
        .ok_or_else(|| anyhow::anyhow!("No reports found"))
        .map(|p| p.clone())
}

