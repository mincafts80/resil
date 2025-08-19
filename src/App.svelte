<script>
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api/tauri";

  let currentTrack = null;
  let queue = [];
  let isPlaying = false;
  let volume = 0.8;
  let progress = 0; // seconds
  let duration = 0; // seconds
  let progressInterval;

  onMount(() => {
    queue = [
      { title: "Track 1", duration: 225 }, // seconds
      { title: "Track 2", duration: 250 },
      { title: "Track 3", duration: 178 }
    ];
  });

  async function play(track) {
    currentTrack = track;
    isPlaying = true;
    progress = 0;
    duration = track.duration;
    startProgress();
    // backend call: await invoke("play_track", { path: track.path });
  }

  function togglePlay() {
    isPlaying = !isPlaying;
    if (isPlaying) startProgress();
    else clearInterval(progressInterval);
  }

  function stop() {
    isPlaying = false;
    currentTrack = null;
    progress = 0;
    duration = 0;
    clearInterval(progressInterval);
  }

  function setVolume(v) {
    volume = v;
    // backend call: await invoke("set_volume", { value: v });
  }

  function startProgress() {
    clearInterval(progressInterval);
    progressInterval = setInterval(() => {
      if (progress < duration && isPlaying) {
        progress++;
      } else {
        clearInterval(progressInterval);
      }
    }, 1000);
  }

  function seek(newTime) {
    progress = newTime;
    // backend call: await invoke("seek", { time: newTime });
  }

  function formatTime(seconds) {
    const m = Math.floor(seconds / 60);
    const s = Math.floor(seconds % 60);
    return `${m}:${s.toString().padStart(2, "0")}`;
  }
</script>

<div class="flex flex-col h-screen bg-gray-900 text-white p-6 space-y-6">
  <!-- Now Playing -->
  <div class="bg-gray-800 rounded-2xl p-6 shadow-xl">
    <h1 class="text-xl font-bold mb-2">Now Playing</h1>
    {#if currentTrack}
      <p class="text-lg">{currentTrack.title}</p>

      <!-- Progress Bar -->
      <div class="mt-4">
        <input
          type="range"
          min="0"
          max={duration}
          step="1"
          bind:value={progress}
          on:change={(e) => seek(parseInt(e.target.value))}
          class="w-full"
        />
        <div class="flex justify-between text-sm text-gray-400 mt-1">
          <span>{formatTime(progress)}</span>
          <span>{formatTime(duration)}</span>
        </div>
      </div>
    {:else}
      <p class="text-gray-500">Nothing playing</p>
    {/if}

    <!-- Controls -->
    <div class="flex items-center space-x-4 mt-4">
      <button
        class="px-4 py-2 bg-blue-600 rounded-xl hover:bg-blue-500"
        on:click={togglePlay}
        disabled={!currentTrack}
      >
        {isPlaying ? "Pause" : "Play"}
      </button>

      <button
        class="px-4 py-2 bg-red-600 rounded-xl hover:bg-red-500"
        on:click={stop}
      >
        Stop
      </button>

      <!-- Volume -->
      <input
        type="range"
        min="0"
        max="1"
        step="0.01"
        bind:value={volume}
        on:input={(e) => setVolume(parseFloat(e.target.value))}
        class="w-32"
      />
      <span class="text-sm">{Math.round(volume * 100)}%</span>
    </div>
  </div>

  <!-- Queue -->
  <div class="bg-gray-800 rounded-2xl p-6 shadow-xl flex-1 overflow-y-auto">
    <h2 class="text-xl font-bold mb-4">Queue</h2>
    {#if queue.length > 0}
      <ul class="space-y-2">
        {#each queue as track, i}
          <li
            class="flex justify-between items-center bg-gray-700 rounded-xl px-4 py-2 hover:bg-gray-600 cursor-pointer"
            on:click={() => play(track)}
          >
            <span>{track.title}</span>
            <span class="text-gray-400 text-sm">{formatTime(track.duration)}</span>
          </li>
        {/each}
      </ul>
    {:else}
      <p class="text-gray-500">Queue is empty</p>
    {/if}
  </div>
</div>
