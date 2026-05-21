use std::path::PathBuf;
use std::process::Stdio;
use tauri::{AppHandle, Emitter};
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Command;

/// Locate a sidecar binary placed next to the running executable.
/// Falls back to the bare name (will resolve via PATH) when the sidecar
/// is not present — useful for `cargo run` outside the Tauri build pipeline.
fn binary_path(name: &str) -> PathBuf {
    #[cfg(target_os = "windows")]
    let filename = format!("{}.exe", name);
    #[cfg(not(target_os = "windows"))]
    let filename = name.to_string();

    if let Ok(exe) = std::env::current_exe() {
        if let Some(parent) = exe.parent() {
            let candidate = parent.join(&filename);
            if candidate.exists() {
                return candidate;
            }
        }
    }
    PathBuf::from(name)
}

fn yt_dlp_path() -> PathBuf {
    binary_path("yt-dlp")
}

fn ffmpeg_path() -> PathBuf {
    binary_path("ffmpeg")
}

#[derive(serde::Serialize, Clone)]
struct ProgressEvent {
    id: String,
    line: String,
}

#[derive(serde::Serialize, Clone)]
struct CompleteEvent {
    id: String,
    success: bool,
    message: String,
}

#[derive(serde::Serialize, Clone, Debug)]
struct VideoMetadata {
    title: String,
    thumbnail: Option<String>,
    duration: Option<f64>,
    channel: Option<String>,
    uploader: Option<String>,
    view_count: Option<u64>,
    video_id: Option<String>,
    extractor: Option<String>,
    webpage_url: Option<String>,
}

#[derive(serde::Serialize, Clone, Debug)]
struct PlaylistEntry {
    id: Option<String>,
    title: String,
    url: Option<String>,
    duration: Option<f64>,
    thumbnail: Option<String>,
    uploader: Option<String>,
    channel: Option<String>,
}

#[derive(serde::Serialize, Clone, Debug)]
struct PlaylistMetadata {
    title: String,
    entry_count: usize,
    entries: Vec<PlaylistEntry>,
    uploader: Option<String>,
    webpage_url: Option<String>,
}

#[derive(serde::Serialize, Clone, Debug)]
#[serde(tag = "kind", rename_all = "snake_case")]
enum FetchResult {
    Video(VideoMetadata),
    Playlist(PlaylistMetadata),
}

fn pick_thumbnail(entry: &serde_json::Value) -> Option<String> {
    if let Some(t) = entry.get("thumbnail").and_then(|v| v.as_str()) {
        return Some(t.to_string());
    }
    entry
        .get("thumbnails")
        .and_then(|v| v.as_array())
        .and_then(|arr| arr.last())
        .and_then(|t| t.get("url"))
        .and_then(|u| u.as_str())
        .map(String::from)
}

#[tauri::command]
async fn fetch_metadata(url: String) -> Result<FetchResult, String> {
    let trimmed = url.trim();
    if trimmed.is_empty() {
        return Err("Empty URL".into());
    }

    let output = Command::new(yt_dlp_path())
        .args([
            "--dump-single-json",
            "--no-warnings",
            "--flat-playlist",
            "--skip-download",
            trimmed,
        ])
        .stderr(Stdio::null())
        .output()
        .await
        .map_err(|e| format!("Failed to run yt-dlp: {}", e))?;

    if !output.status.success() {
        return Err("Could not fetch info — check the URL".into());
    }

    let json: serde_json::Value = serde_json::from_slice(&output.stdout)
        .map_err(|e| format!("Failed to parse metadata: {}", e))?;

    let is_playlist = json.get("_type").and_then(|v| v.as_str()) == Some("playlist");

    if is_playlist {
        let entries: Vec<PlaylistEntry> = json
            .get("entries")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .map(|e| PlaylistEntry {
                        id: e.get("id").and_then(|v| v.as_str()).map(String::from),
                        title: e
                            .get("title")
                            .and_then(|v| v.as_str())
                            .unwrap_or("Untitled")
                            .to_string(),
                        url: e.get("url").and_then(|v| v.as_str()).map(String::from),
                        duration: e.get("duration").and_then(|v| v.as_f64()),
                        thumbnail: pick_thumbnail(e),
                        uploader: e
                            .get("uploader")
                            .and_then(|v| v.as_str())
                            .map(String::from),
                        channel: e.get("channel").and_then(|v| v.as_str()).map(String::from),
                    })
                    .collect()
            })
            .unwrap_or_default();

        let entry_count = entries.len();

        Ok(FetchResult::Playlist(PlaylistMetadata {
            title: json
                .get("title")
                .and_then(|v| v.as_str())
                .unwrap_or("Playlist")
                .to_string(),
            entry_count,
            entries,
            uploader: json
                .get("uploader")
                .and_then(|v| v.as_str())
                .map(String::from),
            webpage_url: json
                .get("webpage_url")
                .and_then(|v| v.as_str())
                .map(String::from),
        }))
    } else {
        Ok(FetchResult::Video(VideoMetadata {
            title: json
                .get("title")
                .and_then(|v| v.as_str())
                .unwrap_or("Untitled")
                .to_string(),
            thumbnail: pick_thumbnail(&json),
            duration: json.get("duration").and_then(|v| v.as_f64()),
            channel: json
                .get("channel")
                .and_then(|v| v.as_str())
                .map(String::from),
            uploader: json
                .get("uploader")
                .and_then(|v| v.as_str())
                .map(String::from),
            view_count: json.get("view_count").and_then(|v| v.as_u64()),
            video_id: json.get("id").and_then(|v| v.as_str()).map(String::from),
            extractor: json
                .get("extractor")
                .and_then(|v| v.as_str())
                .map(String::from),
            webpage_url: json
                .get("webpage_url")
                .and_then(|v| v.as_str())
                .map(String::from),
        }))
    }
}

#[tauri::command]
async fn download_video(
    app: AppHandle,
    id: String,
    url: String,
    format: String,
    output_dir: String,
    subs: bool,
    thumbnail: bool,
) -> Result<(), String> {
    if url.trim().is_empty() {
        return Err("URL is empty".into());
    }
    if output_dir.trim().is_empty() {
        return Err("Output folder not picked".into());
    }

    let mut args: Vec<String> = vec![
        "--newline".into(),
        "--no-warnings".into(),
        "--no-playlist".into(),
        "--add-metadata".into(),
        "-P".into(),
        output_dir.clone(),
        "-o".into(),
        "%(title).200B [%(id)s].%(ext)s".into(),
    ];

    // If the bundled ffmpeg sidecar exists, tell yt-dlp where to find it.
    // (When falling back to PATH lookup, yt-dlp finds ffmpeg itself.)
    let ffmpeg = ffmpeg_path();
    if ffmpeg.is_absolute() && ffmpeg.exists() {
        args.push("--ffmpeg-location".into());
        args.push(ffmpeg.to_string_lossy().to_string());
    }

    let merger_args = "Merger:-c:v copy -c:a aac -b:a 192k";

    match format.as_str() {
        "best" => {
            args.extend([
                "-f".into(),
                "bv*[ext=mp4][vcodec^=avc1]+ba[ext=m4a]/b[ext=mp4]/bv*+ba/b".into(),
                "--merge-output-format".into(),
                "mp4".into(),
                "--postprocessor-args".into(),
                merger_args.into(),
            ]);
        }
        "4k" => {
            args.extend([
                "-f".into(),
                "bv*[height<=2160]+ba/b[height<=2160]".into(),
                "--merge-output-format".into(),
                "mp4".into(),
                "--postprocessor-args".into(),
                merger_args.into(),
            ]);
        }
        "1440" => {
            args.extend([
                "-f".into(),
                "bv*[height<=1440]+ba/b[height<=1440]".into(),
                "--merge-output-format".into(),
                "mp4".into(),
                "--postprocessor-args".into(),
                merger_args.into(),
            ]);
        }
        "1080" => {
            args.extend([
                "-f".into(),
                "bv*[height<=1080][vcodec^=avc1]+ba[ext=m4a]/bv*[height<=1080]+ba/b[height<=1080]".into(),
                "--merge-output-format".into(),
                "mp4".into(),
                "--postprocessor-args".into(),
                merger_args.into(),
            ]);
        }
        "720" => {
            args.extend([
                "-f".into(),
                "bv*[height<=720][vcodec^=avc1]+ba[ext=m4a]/bv*[height<=720]+ba/b[height<=720]".into(),
                "--merge-output-format".into(),
                "mp4".into(),
                "--postprocessor-args".into(),
                merger_args.into(),
            ]);
        }
        "audio" => {
            args.extend(["-x".into(), "--audio-format".into(), "mp3".into()]);
        }
        other => return Err(format!("Unknown format: {}", other)),
    }

    if subs && format != "audio" {
        args.extend([
            "--write-subs".into(),
            "--write-auto-subs".into(),
            "--sub-langs".into(),
            "en.*".into(),
            "--convert-subs".into(),
            "srt".into(),
            "--embed-subs".into(),
        ]);
    }

    if thumbnail {
        args.extend(["--write-thumbnail".into(), "--embed-thumbnail".into()]);
    }

    args.push(url);

    let mut child = Command::new(yt_dlp_path())
        .args(&args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| format!("Failed to spawn yt-dlp: {}", e))?;

    let stdout = child.stdout.take().ok_or("no stdout")?;
    let stderr = child.stderr.take().ok_or("no stderr")?;

    let app_o = app.clone();
    let id_o = id.clone();
    tauri::async_runtime::spawn(async move {
        let mut reader = BufReader::new(stdout).lines();
        while let Ok(Some(line)) = reader.next_line().await {
            let _ = app_o.emit(
                "download-progress",
                ProgressEvent {
                    id: id_o.clone(),
                    line,
                },
            );
        }
    });

    let app_e = app.clone();
    let id_e = id.clone();
    tauri::async_runtime::spawn(async move {
        let mut reader = BufReader::new(stderr).lines();
        while let Ok(Some(line)) = reader.next_line().await {
            let _ = app_e.emit(
                "download-progress",
                ProgressEvent {
                    id: id_e.clone(),
                    line,
                },
            );
        }
    });

    let status = child
        .wait()
        .await
        .map_err(|e| format!("Wait failed: {}", e))?;

    let success = status.success();
    let _ = app.emit(
        "download-complete",
        CompleteEvent {
            id: id.clone(),
            success,
            message: if success {
                "Done".into()
            } else {
                format!("yt-dlp exited with code {:?}", status.code())
            },
        },
    );

    if success {
        Ok(())
    } else {
        Err(format!("yt-dlp exited with code {:?}", status.code()))
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![download_video, fetch_metadata])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
