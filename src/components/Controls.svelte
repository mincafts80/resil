<script>
  import { invoke } from "@tauri-apps/api/core";

  let filePath = "";
  let isPlaying = false;

  async function pickFile() {
    const { open } = await import("@tauri-apps/plugin-dialog");
    filePath = await open({ multiple: false, filters: [{ name: "Audio", extensions: ["mp3", "wav", "ogg", "flac"] }] });
    if (filePath) {
      await invoke("play", { path: filePath });
      isPlaying = true;
    }
  }

  async function togglePlay() {
    if (isPlaying) {
      await invoke("pause");
    } else {
      await invoke("resume");
    }
    isPlaying = !isPlaying;
  }

  async function stop() {
    await invoke("stop");
    isPlaying = false;
  }
</script>

<div class="p-4 bg-gray-900 text-white rounded-2xl shadow-lg flex space-x-4 items-center">
  <button on:click={pickFile} class="bg-blue-500 px-4 py-2 rounded">Open File</button>
  <button on:click={togglePlay} class="bg-green-500 px-4 py-2 rounded">
    {isPlaying ? "Pause" : "Play"}
  </button>
  <button on:click={stop} class="bg-red-500 px-4 py-2 rounded">Stop</button>
</div>

<script lang="ts">
import { invoke } from "@tauri-apps/api/core";
import { loadState, saveState } from "../lib/storage";


let isPlaying = false;
let current: string | null = null;
let volume = loadState().volume ?? 0.8;


async function openFile() {
const { open } = await import("@tauri-apps/plugin-dialog");
const selected = await open({ multiple: false, filters: [{ name: "Audio", extensions: ["mp3","wav","ogg","flac"] }] });
if (typeof selected === "string") {
await invoke("play_now", { path: selected });
await refresh();
persist();
}
}


function persist() { saveState({ volume, lastTrack: current ?? undefined }); }


async function refresh() {
const s = await invoke<any>("get_state");
isPlaying = s.is_playing;
current = s.current;
volume = s.volume;
}


async function togglePlay() {
await invoke(isPlaying ? "pause" : "resume");
await refresh();
}


async function stop() {
await invoke("stop");
await refresh();
}


async function setVol(v: number) {
volume = v;
persist();
const s = await invoke<any>("set_volume", { v: volume });
isPlaying = s.is_playing;
}


// init
refresh();
</script>


<div class="p-4 bg-neutral-800 rounded-2xl shadow flex items-center gap-3">
<button class="px-3 py-2 rounded-xl bg-blue-600" on:click={openFile}>Open file</button>
<button class="px-3 py-2 rounded-xl bg-green-600" on:click={togglePlay}>{isPlaying ? "Pause" : "Play"}</button>
<button class="px-3 py-2 rounded-xl bg-red-600" on:click={stop}>Stop</button>
<div class="flex items-center gap-2 ml-4">
<span class="text-sm opacity-70">Vol</span>
<input type="range" min={0} max={1} step={0.01} bind:value={volume} on:input={(e) => setVol(parseFloat(e.currentTarget.value))} />
</div>
<div class="ml-auto text-sm opacity-80 truncate max-w-[40ch]">{current ?? "No track"}</div>
</div>
