use anyhow::Result;
use serde::Deserialize;

/// The music indicator. (a 'lil gingle: `♪`)
pub const MUSIC_INDICATOR: &str = "♪";

/// Represents a track.
#[derive(Debug, Deserialize)]
pub struct Track {
    /// Track ID.
    pub id: u32,

    /// Name of the track.
    #[serde(rename = "trackName")]
    pub track_name: String,

    /// Name of the artist.
    #[serde(rename = "artistName")]
    pub artist_name: String,

    /// Name of the album.
    #[serde(rename = "albumName")]
    pub album_name: String,

    /// Duration of the track in seconds.
    pub duration: f32,

    /// Whether the track is instrumental.
    pub instrumental: bool,

    /// The plain lyrics without timestamps. May not be present.
    #[serde(rename = "plainLyrics")]
    pub plain_lyrics: Option<String>,

    /// The synced lyrics with timestamps. May not be present.
    #[serde(rename = "syncedLyrics")]
    pub synced_lyrics: Option<SyncedLyrics>,
}

/// Represents a timestamp, consisting of minutes, seconds, and milliseconds.
#[derive(Debug, Clone)]
pub struct Timestamp {
    /// Minutes.
    pub m: u32,

    /// Seconds.
    pub s: u8,

    /// Milliseconds.
    pub ms: u32,
}

impl Timestamp {
    /// Creates a new instance of `Timestamp`.
    pub fn new(m: u32, s: u8, ms: u32) -> Self {
        Self { m, s, ms }
    }

    pub fn seconds(&self) -> f32 {
        (self.m * 60 + self.s as u32 + self.ms / 1000) as f32
    }
}

#[derive(Debug)]
pub struct SyncedLyrics {
    /// The raw synced lyrics from the API.
    pub raw: String,
}

impl SyncedLyrics {
    /// Creates a new instance of `SyncedLyrics`.
    pub fn new(raw: String) -> Self {
        Self { raw }
    }

    /// Returns the synced lyrics as a vector of `(timestamp, lyrics)`.
    pub fn pieces(&self) -> Vec<(Timestamp, String)> {
        let re = regex::Regex::new(r"\[(\d+):(\d+).(\d+)\][ ]*(.*)").unwrap();

        re.captures_iter(&self.raw)
            .map(|cap| {
                (
                    Timestamp::new(
                        cap.get(1).unwrap().as_str().parse::<u32>().unwrap(),
                        cap.get(2).unwrap().as_str().parse::<u8>().unwrap(),
                        cap.get(3).unwrap().as_str().parse::<u32>().unwrap(),
                    ),
                    {
                        let item = cap.get(4).unwrap().as_str().to_string();
                        if item.trim().is_empty() {
                            MUSIC_INDICATOR.to_string()
                        } else {
                            item
                        }
                    },
                )
            })
            .collect::<Vec<_>>()
    }

    /// Gets the nearest lyrics to the given timestamp.
    /// Returns: (timestamp, lyrics)
    pub fn at(&self, timestamp: Timestamp) -> (Timestamp, String) {
        let pieces = self.pieces();
        let mut elapsed = 0_f32;
        let mut i = 0_usize;

        while elapsed < timestamp.seconds() {
            elapsed += pieces[i].0.seconds();
            i += 1;
        }

        let (timestamp, lyrics) = &pieces[i];

        (timestamp.clone(), lyrics.clone())
    }

    /// Calculates the delta between each timestamp.
    /// # Arguments
    /// * `include_initial` - Whether to include the initial delta. (like waiting for preludes)
    pub fn deltas(&self, include_initial: bool) -> Vec<f32> {
        let pieces = self.pieces();
        let mut deltas = vec![];

        if include_initial {
            deltas.push(pieces[0].0.seconds());
        }

        let mut i = 0_usize;
        while i < pieces.len() - 1 {
            deltas.push(pieces[i + 1].0.seconds() - pieces[i].0.seconds());
            i += 1;
        }

        deltas
    }
}

impl<'de> Deserialize<'de> for SyncedLyrics {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let raw = String::deserialize(deserializer)?;
        Ok(SyncedLyrics::new(raw))
    }
}

/// Search for a track.
///
/// ```rust
/// let results: Vec<Track> = search("slow dancing in the dark").await?;
/// println!("{:#?}", results);
/// ```
pub async fn search<K: ToString>(q: K) -> Result<Vec<Track>> {
    let client = reqwest::Client::new();
    let resp = client
        .get("https://lrclib.net/api/search")
        .query(&[("q", q.to_string())])
        .send()
        .await?;

    Ok(resp.json::<Vec<Track>>().await?)
}

/// Get a track by its ID.
///
/// ```rust
/// let track = get_by_id(1).await?;
/// println!("{:#?}", track);
pub async fn get_by_id(id: u32) -> Result<Track> {
    let client = reqwest::Client::new();
    let resp = client
        .get(format!("https://lrclib.net/api/get/{}", id))
        .send()
        .await?;

    Ok(resp.json::<Track>().await?)
}
