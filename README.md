# tlyrics
Fetch synced lyrics (with timestamps) from [LRCLib](https://lrclib.net) using the LRC format.

## Search
To search for a track, you can use `search()`:

```rust
let results = search("slow dancing in the dark").await?;
println!("{:#?}", results);

let song = results[0];

println!("Name: {}", song.track_name);
println!("By: {}", song.artist_name);
println!("Album: {}", song.album_name);
println!("Duration: {}", song.duration);

// Synced lyrics
if let Some(lyrics) = song.synced_lyrics {
    println!("{:#?}", lyrics.pieces());
}
```

## By ID
To get a track by its ID, you can use `get_by_id()`:

```rust
let song = get_by_id(song_id).await?;

// ... logic
```

***

[AWeirdDev](https://github.com/AWeirdDev)
