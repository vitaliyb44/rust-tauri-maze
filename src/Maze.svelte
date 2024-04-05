<script lang="ts">
    import { invoke } from "@tauri-apps/api";
    import { listen, type Event, type UnlistenFn } from "@tauri-apps/api/event";
    import { onMount } from "svelte";
    import { onDestroy } from "svelte";
    import {
        DURATION,
        keyboardMenu,
        loading,
        mazeLoaded,
        MAZE_HEIGHT,
        MAZE_SIZE,
        MAZE_WIDTH,
        menuOpen,
        STEP_FLAG,
        VIEW_PORT_SIZE,
    } from "./appConstants";
    interface Cell {
        visited: boolean;
        wall_e: boolean;
        wall_n: boolean;
        wall_s: boolean;
        wall_w: boolean;
    }

    import Cell from "./Cell.svelte";

    let currentMaze: Cell[][] = [[]];

    function populateMaze(h: number, w: number) {
        let newMaze = [];
        for (let i = 0; i < h; i++) {
            let row = [];
            for (let y = 0; y < w; y++) {
                row.push({
                    visited: false,
                    wall_e: true,
                    wall_n: true,
                    wall_s: true,
                    wall_w: true,
                });
            }
            newMaze.push(row);
        }
        return newMaze;
    }

    function updateMaze(
        maze_cell_e: Cell,
        maze_cell_s: Cell,
        index1: { x: number; y: number },
        index2: { x: number; y: number }
    ) {
        currentMaze[index1.y][index1.x] = { ...maze_cell_e };
        currentMaze[index2.y][index2.x] = { ...maze_cell_s };

        currentMaze = currentMaze;
    }

    let unlisten: UnlistenFn;
    let unlisten2: UnlistenFn;
    let unlisten3: UnlistenFn;

    onMount(async () => {
        unlisten3 = await listen("maze_generate_start", () => {
            currentMaze = populateMaze($MAZE_HEIGHT, $MAZE_WIDTH);
        });
        unlisten2 = await listen("maze_finished", (event: Event<Cell[][]>) => {
            if (!$STEP_FLAG) {
                currentMaze = [...event.payload];
                currentMaze = currentMaze;
            }
        });
        unlisten = await listen("add_maze_point", (e: any) => {
            let [[maze_cell_e, index1], [maze_cell_s, index2]] = [...e.payload];
            updateMaze(maze_cell_e, maze_cell_s, index1, index2);
        });
        console.log(currentMaze[0]);
    });

    onDestroy(async () => {
        unlisten();
        unlisten2();
        unlisten3();
    });
</script>

<div
    class="maze-container"
    style="grid-template-columns: repeat({currentMaze[0].length}, 1fr);
        grid-template-rows: repeat({currentMaze.length}, 1fr);"
>
    {#each currentMaze as mazeRow, y}
        {#each mazeRow as cell, i (i + currentMaze.length * y)}
            <Cell {cell} />
        {/each}
    {/each}
</div>

<style>
    .maze-container {
        height: 100%;
        width: 100%;
        display: grid;

        background-color: black;
    }
</style>
