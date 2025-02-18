#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

#[tokio::main]
async fn main() -> Result<(), eframe::Error> {
    femme::with_level(femme::LevelFilter::Info);

    gui::run().await?;
    Ok(())
}
