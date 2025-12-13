use chromiumoxide::browser::Browser;
use futures_util::StreamExt;
use std::process::Command;
use tokio::time::{sleep, Duration};

/// Launch Chrome manually with a dedicated profile and connect via DevTools for interactive testing.
///
/// This mirrors the demo from the upstream repository but is factored into the chrome client module
/// so `main` stays small and focused on orchestration.
pub async fn launch_manual_devtools_session() -> anyhow::Result<()> {
    // Use a dedicated Chrome profile for this project
    // Login once through the UI, and future runs will be logged in automatically
    let home = std::env::var("HOME")?;
    let chrome_profile_dir = format!("{}/.local/share/news_agent/chrome_profile", home);

    // Create the directory if it doesn't exist
    std::fs::create_dir_all(&chrome_profile_dir)?;

    // Launch Chrome manually with remote debugging, WITHOUT --enable-automation flag
    // This prevents the automation banner from appearing
    let remote_debugging_port = 9222;
    let chrome_exe = "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome";

    println!("🚀 Launching Chrome manually (without automation flag)...");
    Command::new(chrome_exe)
        .arg(&format!("--remote-debugging-port={}", remote_debugging_port))
        .arg(&format!("--user-data-dir={}", chrome_profile_dir))
        .arg("--no-first-run")
        .arg("--no-default-browser-check")
        .arg("--enable-automation") // add for testing, remove for production
        .spawn()?;

    // Wait a moment for Chrome to start
    sleep(Duration::from_millis(2000)).await;

    // Connect to the manually launched Chrome instance
    let (browser, mut handler) = Browser::connect(format!("http://127.0.0.1:{}", remote_debugging_port)).await?;

    // Drive the CDP event loop
    tokio::spawn(async move {
        while let Some(_event) = handler.next().await {}
    });

    let _page = browser.new_page("https://accounts.google.com").await?;

    println!("👉 Browser opened with dedicated profile. Log in once, and future runs will be logged in automatically.");
    println!("   Profile location: {}", chrome_profile_dir);
    println!("   Press Ctrl+C to exit.");
    sleep(Duration::from_secs(600)).await;

    Ok(())
}
