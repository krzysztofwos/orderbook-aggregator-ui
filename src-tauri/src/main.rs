#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use anyhow::{Context, Result};
use clap::Parser;
use tauri::Manager;
use tokio_stream::StreamExt;

use orderbook_aggregator::orderbook::{
    orderbook_aggregator_client::OrderbookAggregatorClient, Empty, Summary,
};

/// Orderbook Aggregator UI
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Server URL
    #[arg(long, default_value = "htttp://0.0.0.0:50051")]
    url: String,
}

fn emit_summary_event<R: tauri::Runtime>(manager: &impl Manager<R>, summary: Summary) {
    manager.emit_all("summary", summary).unwrap();
}

fn main() -> Result<()> {
    let args = Args::parse();
    tauri::Builder::default()
        .setup(|app| {
            let app_handle = app.handle();
            tauri::async_runtime::spawn(async move {
                let mut client = OrderbookAggregatorClient::connect(args.url).await.unwrap();
                let mut stream = client.book_summary(Empty {}).await.unwrap().into_inner();

                while let Some(item) = stream.next().await {
                    let summary = item.unwrap();
                    emit_summary_event(&app_handle, summary);
                }
            });
            Ok::<(), Box<dyn std::error::Error + 'static>>(())
        })
        .run(tauri::generate_context!())
        .context("Tauri application error")
}
