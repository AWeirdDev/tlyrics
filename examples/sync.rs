use std::time::Duration;

use anyhow::Result;
use tlyrics::{get_by_id, MUSIC_INDICATOR};

#[tokio::main]
async fn main() -> Result<()> {
    let track = get_by_id(5432440).await?;

    if let Some(lyrics) = track.synced_lyrics {
        println!("{:?}", lyrics.pieces());
        let deltas = lyrics.deltas(true);

        println!("{}", MUSIC_INDICATOR); // prelude
        for (i, (timestamp, lyrics)) in lyrics.pieces().iter().enumerate() {
            std::thread::sleep(Duration::from_secs_f32(deltas[i]));
            println!("{:?} - {}", timestamp, lyrics);
        }
    }

    Ok(())
}
