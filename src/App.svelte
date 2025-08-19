<script>
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api/tauri";

  let currentTrack = null;
  let queue = [];
  let isPlaying = false;
  let volume = 0.8;

  // Dummy data (replace with real backend calls later)
  onMount(() => {
    queue = [
      { title: "Track 1", duration: "3:45" },
      { title: "Track 2", duration: "4:10" },
      { title: "Track 3", duration: "2:58" }
    ];
  });

  async function play(track) {
    currentTrack = track;
    isPlaying = true;
    // backend call: await invoke("play_track", { path: track.path });
  }

  function togglePlay() {
    isPlaying = !isPlaying;
  }

  function stop() {
    isPlaying = false;
    currentTrack = null;
  }

  function setVolume(v) {
    volume = v;
    // backend call: await invoke("set_volume", { value: v });
  }
</script>

<div class="flex flex-col h-screen bg-gray-900 text-white p-6 space-y-6">
  <!-- Now Playing -->
  <div class="bg-gray-800 rounded-2xl p-6 shadow-xl">
    <h1 class="text-xl font-bold mb-2">Now Playing</h1>
    {#if currentTrack}
      <p class="text-lg">{currentTrack.title}</p>
      <p class="text-sm text-gray-400">{currentTrack.duration}</p>
    {:else}
      <p class="text-gray-500">Nothing playing</p>
    {/if}

    <!-- Controls -->
    <div class="flex items-center space-x-4 mt-4">
      <button
        class="px-4 py-2 bg-blue-600 rounded-xl hover:bg-blue-500"
        on:click={togglePlay}
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
            <span class="text-gray-400 text-sm">{track.duration}</span>
          </li>
        {/each}
      </ul>
    {:else}
      <p class="text-gray-500">Queue is empty</p>
    {/if}
  </div>
</div>

<style>
  /* Custom scrollbar for queue */
  .queue::-webkit-scrollbar {
    width: 6px;
  }
  .queue::-webkit-scrollbar-thumb {
    background: #555;
    border-radius: 3px;
  }
</style>
