<script lang="ts">
import { invoke } from "@tauri-apps/api/core";


let queue: string[] = [];
let current: string | null = null;


async function refresh() {
const s = await invoke<any>("get_state");
queue = s.queue;
current = s.current;
}


async function addToQueue() {
const { open } = await import("@tauri-apps/plugin-dialog");
const sel = await open({ multiple: true, filters: [{ name: "Audio", ext