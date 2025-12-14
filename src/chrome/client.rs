use chromiumoxide::browser::Browser;
use chromiumoxide::page::Page;
use futures_util::StreamExt;
use std::process::Command;
use tokio::time::{sleep, Duration};
use anyhow::{Context, Result};

/// Chrome DevTools client wrapper for fetching HTML content from web pages.
pub struct ChromeClient {
    browser: Browser,
    remote_debugging_port: u16,
}

impl ChromeClient {
    /// Connects to an existing Chrome instance running with remote debugging.
    pub async fn connect(remote_debugging_port: u16) -> Result<Self> {
        let (browser, mut handler) = Browser::connect(format!("http://127.0.0.1:{}", remote_debugging_port))
            .await
            .context("Failed to connect to Chrome DevTools")?;

        // Drive the CDP event loop
        tokio::spawn(async move {
            while let Some(_event) = handler.next().await {}
        });

        Ok(Self {
            browser,
            remote_debugging_port,
        })
    }

    /// Launches Chrome manually with a dedicated profile and connects to it.
    /// This is useful for interactive testing and initial setup.
    pub async fn launch_manual_devtools_session() -> Result<()> {
        let home = std::env::var("HOME")?;
        let chrome_profile_dir = format!("{}/.local/share/news_agent/chrome_profile", home);

        // Create the directory if it doesn't exist
        std::fs::create_dir_all(&chrome_profile_dir)?;

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

        let _client = Self::connect(remote_debugging_port).await?;
        let _page = _client.browser.new_page("https://accounts.google.com").await?;

        println!("👉 Browser opened with dedicated profile. Log in once, and future runs will be logged in automatically.");
        println!("   Profile location: {}", chrome_profile_dir);
        println!("   Press Ctrl+C to exit.");
        sleep(Duration::from_secs(600)).await;

        Ok(())
    }

    /// Fetches the HTML content of a URL by opening a page, waiting for network idle, and extracting the DOM.
    /// This is the core reusable helper for all source modules.
    pub async fn fetch_html(&self, url: &str) -> Result<String> {
        let page = self.browser.new_page(url).await
            .with_context(|| format!("Failed to open page: {}", url))?;

        // Wait for network idle to ensure content is loaded
        sleep(Duration::from_secs(2)).await;

        let html = page.content().await
            .with_context(|| format!("Failed to get page content: {}", url))?;

        // Close the page
        page.close().await.ok();

        Ok(html)
    }

    /// Returns a reference to the underlying browser instance (for advanced use cases).
    pub fn browser(&self) -> &Browser {
        &self.browser
    }
}
