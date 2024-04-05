import { writable } from "svelte/store";

const MAZE_HEIGHT = writable(10);
const MAZE_WIDTH = writable(10);
const STEP_FLAG = writable(false);
const DURATION = writable(100);

const mazeLoaded = writable(false);
const menuOpen = writable(false);
const loading = writable(false);

const VIEW_PORT_SIZE = 1000;
const MAZE_SIZE = 1000;

const keyboardMenu = "Escape"

export { MAZE_HEIGHT, MAZE_WIDTH, VIEW_PORT_SIZE, MAZE_SIZE, keyboardMenu, loading, STEP_FLAG, DURATION, mazeLoaded, menuOpen }