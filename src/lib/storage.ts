export type SavedState = { volume: number; lastTrack?: string };


const KEY = "resilience.player.state";


export function loadState(): SavedState {
try {
const raw = localStorage.getItem(KEY);
if (!raw) return { volume: 0.8 };
return JSON.parse(raw);
} catch {
return { volume: 0.8 };
}
}


export function saveState(s: SavedState) {
localStorage.setItem(KEY, JSON.stringify(s));
}