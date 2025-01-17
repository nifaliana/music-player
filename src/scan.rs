use futures::future::FutureExt;
use music_player_entity::{album, artist, artist_tracks, track};
use music_player_scanner::scan_directory;
use music_player_scanner::types::Song;
use music_player_storage::Database;
use owo_colors::OwoColorize;
use sea_orm::{ActiveModelTrait, ActiveValue, EntityTrait};

pub async fn auto_scan_music_library() {
    let db = Database::new().await;
    match track::Entity::find().all(db.get_connection()).await {
        Ok(result) => {
            if result.len() == 0 {
                scan_music_library(false).await.unwrap_or_default();
            }
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}

pub async fn scan_music_library(enable_log: bool) -> Result<Vec<Song>, lofty::error::LoftyError> {
    scan_directory(move |song, db| {
        async move {
            let id = format!("{:x}", md5::compute(song.artist.to_string()));
            let item = artist::ActiveModel {
                id: ActiveValue::set(id),
                name: ActiveValue::Set(song.artist.clone()),
            };
            match item.insert(db.get_connection()).await {
                Ok(_) => (),
                Err(_) => (),
            }

            let id = format!("{:x}", md5::compute(format!("{}", song.album)));
            let item = album::ActiveModel {
                id: ActiveValue::set(id),
                title: ActiveValue::Set(song.album.clone()),
                artist: ActiveValue::Set(song.artist.clone()),
                artist_id: ActiveValue::Set(Some(format!(
                    "{:x}",
                    md5::compute(song.artist.to_string())
                ))),
                year: ActiveValue::Set(song.year),
                cover: ActiveValue::Set(song.cover.clone()),
            };
            match item.insert(db.get_connection()).await {
                Ok(_) => (),
                Err(_) => (),
            }
            let id = format!("{:x}", md5::compute(song.uri.as_ref().unwrap()));
            let item = track::ActiveModel {
                id: ActiveValue::set(id),
                title: ActiveValue::Set(song.title.clone()),
                genre: ActiveValue::Set(song.genre.clone()),
                year: ActiveValue::Set(song.year),
                track: ActiveValue::Set(song.track),
                bitrate: ActiveValue::Set(song.bitrate),
                sample_rate: ActiveValue::Set(song.sample_rate),
                bit_depth: ActiveValue::Set(song.bit_depth),
                channels: ActiveValue::Set(song.channels),
                duration: ActiveValue::Set(Some(song.duration.as_secs_f32())),
                uri: ActiveValue::Set(song.uri.clone().unwrap_or_default()),
                album_id: ActiveValue::Set(Some(format!(
                    "{:x}",
                    md5::compute(format!("{}", song.album))
                ))),
                artist_id: ActiveValue::Set(Some(format!(
                    "{:x}",
                    md5::compute(song.artist.to_string())
                ))),
            };

            if enable_log {
                let filename = song.uri.as_ref().unwrap().split("/").last().unwrap();
                let path = song.uri.as_ref().unwrap().replace(filename, "");
                println!("{}{}", path, filename.magenta());
            }

            match item.insert(db.get_connection()).await {
                Ok(_) => (),
                Err(_) => (),
            }

            let item = artist_tracks::ActiveModel {
                id: ActiveValue::set(format!(
                    "{:x}",
                    md5::compute(format!("{}{}", song.artist, song.uri.as_ref().unwrap()))
                )),
                artist_id: ActiveValue::Set(format!("{:x}", md5::compute(song.artist.to_string()))),
                track_id: ActiveValue::Set(format!(
                    "{:x}",
                    md5::compute(song.uri.as_ref().unwrap())
                )),
            };
            match item.insert(db.get_connection()).await {
                Ok(_) => (),
                Err(_) => (),
            }
        }
        .boxed()
    })
    .await
}
