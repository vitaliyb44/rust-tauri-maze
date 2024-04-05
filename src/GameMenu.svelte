<script lang="ts">
    import { fade, fly, slide } from "svelte/transition";
    import { cubicInOut } from "svelte/easing";
    let menuItems = [
        {
            menuName: "Generate Maze",
            options: {
                input: "range",
            },
        },
        { menuName: "Quit", options: {} },
    ];
    import { createEventDispatcher } from "svelte";
    import {
        DURATION,
        MAZE_HEIGHT,
        MAZE_WIDTH,
        STEP_FLAG,
    } from "./appConstants";

    const dispatch = createEventDispatcher<{ menu: { item: string } }>();

    function handleMenu(item: string) {
        dispatch("menu", {
            item,
        });
    }
</script>

<aside class="menu-wrapper">
    <div
        class="menu"
        in:fly={{ y: 500, x: 0, duration: 500, easing: cubicInOut }}
        out:fly={{
            y: 500,
            x: 0,
            duration: 500,
            easing: cubicInOut,
            delay: 100,
        }}
    >
        <div class="menu-items">
            {#each menuItems as menuItem, i}
                <div
                    class="menu-item {menuItem.menuName
                        .replaceAll(' ', '')
                        .toLocaleLowerCase()}"
                    in:fly={{ x: -500, delay: i * 300 }}
                    out:fly={{ x: -500 }}
                >
                    <button
                        on:click|stopPropagation={() => {
                            handleMenu(menuItem.menuName);
                        }}
                    >
                        {menuItem.menuName}
                    </button>
                    {#if menuItem.menuName == "Generate Maze"}
                        <div class="menu-options">
                            <div class="menu-options-form">
                                <div class="range-input">
                                    <div class="label">Width</div>
                                    <div class="value">{$MAZE_WIDTH}</div>
                                    <input
                                        type="range"
                                        min="10"
                                        max="100"
                                        bind:value={$MAZE_WIDTH}
                                    />
                                </div>

                                <div class="range-input">
                                    <div class="label">Height</div>
                                    <div class="value">{$MAZE_HEIGHT}</div>
                                    <input
                                        type="range"
                                        min="10"
                                        max="100"
                                        bind:value={$MAZE_HEIGHT}
                                    />
                                </div>
                                <div class="flag-input">
                                    <label class="label" for="simulate"
                                        >Show steps</label
                                    >
                                    <input
                                        class="input-checkbox"
                                        id="simulate"
                                        type="checkbox"
                                        bind:checked={$STEP_FLAG}
                                    />
                                </div>
                                {#if $STEP_FLAG}
                                    <div class="duration-input">
                                        <label class="label" for="duration"
                                            >Time between steps</label
                                        >
                                        <input
                                            class="duration-input-input"
                                            type="text"
                                            name="duration"
                                            id="duration"
                                            bind:value={$DURATION}
                                        />
                                    </div>
                                {/if}
                            </div>
                        </div>
                    {/if}
                </div>
            {/each}
        </div>
    </div>
</aside>

<style>
    .menu-wrapper {
        position: absolute;
        width: 100%;
        height: 100%;
        z-index: 99;
        color: white;
        background-color: rgba(0, 0, 0, 0.2);
    }
    .menu {
        box-shadow: 1rem 1rem 0rem black;
        border: 2px solid black;
        background-color: rgb(71, 60, 95);
        border-top-left-radius: 0.5rem;
        border-top-right-radius: 0.5rem;
        margin: 2rem;
        height: calc(100%);
        width: calc(50% - 4rem);
    }
    .menu-items {
        overflow-y: auto;
        height: 100%;
        display: flex;
        flex-direction: column;
    }
    .menu-item {
        display: flex;
        flex-direction: column;
    }
    .menu-item.quit {
        align-self: flex-start;
    }

    .menu-options {
        border: 2px solid black;
        border-top: none;
        border-end-end-radius: 0.25rem;
        border-end-start-radius: 0.25rem;
        background-color: gray;
        margin: 0 1rem 1rem 1rem;
        align-self: stretch;
    }
    .menu-options-form {
        display: flex;
        flex-direction: column;
        padding: 1rem 0;
    }
    .range-input,
    .flag-input,
    .duration-input {
        display: flex;
        flex-wrap: wrap;
    }
    .flag-input,
    .duration-input {
        align-items: center;
        justify-content: space-between;
    }
    .flag-input {
        padding: 1rem 0;
    }
    .flag-input .label {
        width: 50%;
    }

    .input-checkbox {
        /* width: 50%; */
        margin: 0 1rem;
    }

    .duration-input-input {
        text-align: end;
        width: 25%;
        margin: 0 1rem;
    }

    .label {
        font-size: large;
        width: 50%;
    }
    .label,
    .value {
        padding: 0rem 1rem;
    }
    .range-input .value {
        width: 50%;
        text-align: end;
    }
    .range-input input {
        width: 100%;
        padding: 1rem 0rem;
        margin: 0 1rem;
    }

    button {
        border: 2px solid black;

        background-color: black;
        font-size: 4vw;
        padding: 1rem 2rem;
        margin: 1rem;
        border-radius: 0.25rem;
        color: white;
    }
    button:has(+ .menu-options) {
        border-end-end-radius: 0;
        border-end-start-radius: 0;
        margin: 1rem 1rem 0 1rem;
    }
    button:hover {
        cursor: pointer;
        color: black;
        background-color: white;
    }
</style>
