use crate::api_response::{Track};

pub fn print_tracks(tracks: Vec<&Track>) {
    for track in tracks {
        println!("🔥 {}", track.name);
        println!("💿 {}", track.album.name);
        println!(
            "🕺 {}",
            track
                .album
                .artists
                .iter()
                .map(|artist| artist.name.to_string())
                .collect::<String>()
        );
        println!("🌎 {}", track.external_urls.spotify);
        println!("---------")
    }
}