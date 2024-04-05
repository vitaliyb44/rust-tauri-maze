<script lang="ts">
  import { invoke } from "@tauri-apps/api";

  import Maze from "./Maze.svelte";
  import GameLayout from "./GameLayout.svelte";
  import GameMenu from "./GameMenu.svelte";

  import {
    DURATION,
    keyboardMenu,
    menuOpen,
    loading,
    MAZE_HEIGHT,
    MAZE_WIDTH,
    STEP_FLAG,
  } from "./appConstants";
  import { onDestroy, onMount } from "svelte";
  import { listen, type UnlistenFn } from "@tauri-apps/api/event";

  let unlisten: UnlistenFn;
  let unlisten2: UnlistenFn;
  let unlisten3: UnlistenFn;

  onMount(async () => {
    unlisten2 = await listen("maze_finished", () => {
      $loading = false;
    });
  });

  onDestroy(async () => {
    unlisten2();
  });

  async function handleMenu(e: CustomEvent<{ item: string }>) {
    let item = e.detail.item
      .toLocaleLowerCase()
      .split("")
      .filter((item) => item != " ")
      .join("");
    switch (item) {
      case "generatemaze":
        $menuOpen = false;
        await invoke("create_maze", {
          rows: $MAZE_HEIGHT,
          cols: $MAZE_WIDTH,
          stepFlag: $STEP_FLAG,
          sleepDuration: parseInt($DURATION.toString()),
        }).then(() => {
          $loading = true;
        });
        break;
      default:
        return;
    }
  }

  function handleKeyboard(e: KeyboardEvent) {
    switch (e.key) {
      case keyboardMenu:
        $menuOpen = !$menuOpen;
        break;
      case "z":
        (async () => {
          await invoke("pause_maze_generation");
        })();
        break;
      default:
        break;
    }
  }
</script>

{#if $menuOpen}
  <GameMenu on:menu={handleMenu} />
{/if}
<main>
  <GameLayout>
    {#if $loading}
      <div class="prompt">Loading...</div>
    {/if}
    <Maze />
  </GameLayout>
</main>

<svelte:body on:keydown={handleKeyboard} />

<style>
  main {
    width: 100%;
    height: 100%;
    display: flex;
    justify-content: center;
    align-items: center;
  }
  .prompt {
    z-index: 80;

    position: absolute;
    text-transform: uppercase;
    margin: auto;
    display: flex;
    align-items: center;
    justify-content: center;

    border: 1px solid black;
    border-radius: 0.25rem;
    padding: 3rem;
    color: white;
    background-color: rgb(71, 60, 95);

    width: 50%;
    height: 2em;
    font-size: 2.5vw;
  }
</style>
