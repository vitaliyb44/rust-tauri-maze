<script lang="ts">
    import { onMount } from "svelte";
    import { fade } from "svelte/transition";

    interface Cell {
        visited: boolean;
        wall_e: boolean;
        wall_n: boolean;
        wall_s: boolean;
        wall_w: boolean;
    }
    export let cell: Cell;
    let clicked = false;
    onMount(() => {
        // console.log(cell);
    });

    $: classList = `${cell.wall_n ? "top" : ""} ${
        cell.wall_s ? "bottom" : ""
    } ${cell.wall_w ? "left" : ""} ${cell.wall_e ? "right" : ""}`;
</script>

<div transition:fade class="cell {classList} {clicked ? 'path' : ''}" />

<style>
    .cell {
        display: flex;
        justify-content: center;
        align-items: center;
        /* font-size: 50%; */
        background-color: #454545;
        position: relative;
        border: 0rem solid black;

        /* border: 0.25rem solid black; */
    }
    /* .wall {
        background-color: red;
    } */

    .path {
        background-color: lightblue;
    }
    .top {
        border-top: 0.125rem solid black;
        /* background-color: indigo; */
    }
    .right {
        border-right: 0.125rem solid black;
        /* background-color: orangered; */
    }
    .bottom {
        border-bottom: 0.125rem solid black;
        /* background-color: lime; */
    }
    .left {
        border-left: 0.125rem solid black;
        /* background-color: cyan; */
    }
    /* .left.right {
        background-color: red;
    }
    .top.bottom {
        background-color: green;
    }
    .right.top {
        background-color: lightblue;
    }
    .left.top {
        background-color: purple;
    }
    .right.bottom {
        background-color: orange;
    }
    .left.bottom {
        background-color: yellow;
    }

    .top.left.bottom {
        background-color: #454545;
    } */
</style>
