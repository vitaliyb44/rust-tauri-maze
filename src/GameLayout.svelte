<div class="game-container">
    <div class="top-border" />
    <div class="left-border" />
    <div class="maze-wrapper">
        <slot />
    </div>
    <div class="right-border" />
    <div class="bottom-border" />
</div>

<style>
    .game-container {
        height: 100%;
        width: 100%;
        display: grid;
        grid-template-columns: 2.5% 1fr 2.5%;
        grid-template-rows: 2.5% 1fr 2.5%;
    }
    .maze-wrapper {
        display: flex;
        justify-content: center;
        align-items: center;
    }
    .top-border {
        grid-column: span 3;
    }

    .bottom-border {
        grid-column: span 3;
    }
    .game-container:not(.maze) {
        background-color: darkgray;
    }
</style>
