
use clock::clock::clock;
use counter::counter::counter;
use iced::Settings;
mod counter;
mod clock;
mod Multiwindow;


#[tokio::main]
async fn main() -> iced::Result {
    Multiwindow::window1::window1_start();
    Ok(())
}
