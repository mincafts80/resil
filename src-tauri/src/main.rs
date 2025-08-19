use once_cell::sync::Lazy;
use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink, Source};
use serde::{Deserialize, Serialize};
use std::{
collections::VecDeque,
fs::File,
io::BufReader,
path::PathBuf,
sync::{Arc, Mutex},
thread,
time::Duration,
};
use tauri::{Manager, State};


#[derive(Default)]
struct PlayerCore {
sink: Option<Sink>,
stream: Option<OutputStream>,
handle: Option<OutputStreamHandle>,
queue: VecDeque<PathBuf>,
current: Option<PathBuf>,
volume: f32, // 0.0..=1.0
}


static PLAYER: Lazy<Arc<Mutex<PlayerCore>>> = Lazy::new(|| Arc::new(Mutex::new(PlayerCore { volume: 0.8, ..Default::default() })));


#[derive(Serialize)]
struct PlayerStateDTO {
is_playing: bool,
current: Option<String>,
queue: Vec<String>,
volume: f32,
}


#[derive(Deserialize)]
struct EnqueuePayload { path: String, play_now: Option<bool> }


fn ensure_output(core: &mut PlayerCore) -> Result<(), String> {
if core.stream.is_none() || core.handle.is_none() {
let (stream, handle) = OutputStream::try_default().map_err(|e| e.to_string())?;
core.stream = Some(stream);
core.handle = Some(handle);
}
if core.sink.is_none() {
let sink = Sink::try_new(core.handle.as_ref().unwrap()).map_err(|e| e.to_string())?;
sink.set_volume(core.volume);
core.sink = Some(sink);
}
Ok(())
}


fn start_watcher(app: tauri::AppHandle) {
let player = Arc::clone(&*PLAYER);
thread::spawn(move || loop {
thread::sleep(Duration::from_millis(300));
let mut next_to_play: Option<PathBuf> = None;
{
let mut core = player.lock().unwrap();
if let Some(sink) = &core.sink {
// If finished and queue has more, pop next
if sink.empty() {
if let Some(next) = core.queue.pop_front() {
next_to_play = Some(next);
}
}
}
}