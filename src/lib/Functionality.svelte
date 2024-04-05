<!-- <script lang="ts">
    let zoom = "1.0";
    $: value = calculateOffset(zoom);

    let zoomAmount = 0.1;
    let startedDrag = false;

    let dragAmountX = 0;
    let dragAmountY = 0;

    let mouseStartX = 0;
    let mouseStartY = 0;

    import {
        keyboardMenu,
        MAZE_HEIGHT,
        MAZE_SIZE,
        MAZE_WIDTH,
        VIEW_PORT_SIZE,
    } from "../appConstants";
    
    function handleMenu(e: CustomEvent<{ item: string }>) {
        let item = e.detail.item
            .toLocaleLowerCase()
            .split("")
            .filter((item) => item != " ")
            .join("");
        switch (item) {
            case "generatemaze":
                (async () => {
                    await invoke("create_maze", {
                        rows: MAZE_HEIGHT,
                        cols: MAZE_WIDTH,
                    }).then((data: Cell[][]) => {
                        console.log("generated maze");
                        currentMaze = data;
                        currentMaze = currentMaze;
                    });
                    menuOpen = false;
                })();
                break;
            default:
                return;
        }
    }
    function handleDragging(e: MouseEvent) {
        if (menuOpen || currentMaze.length == 0) return;
        if (startedDrag) {
            let mouseDeltaX = mouseStartX - e.clientX;
            let mouseDeltaY = mouseStartY - e.clientY;

            dragAmountY += mouseDeltaY;
            dragAmountX += mouseDeltaX;

            mouseStartX = e.clientX;
            mouseStartY = e.clientY;
        }
    }
    function handleDragStart(e: MouseEvent) {
        if (menuOpen || currentMaze.length == 0) return;
        mouseStartX = e.clientX;
        mouseStartY = e.clientY;

        startedDrag = true;
    }
    function handleDragEnd(e: MouseEvent) {
        if (menuOpen || currentMaze.length == 0) return;

        startedDrag = false;
        const { fixedX, fixedY } = fixDragAmount(
            dragAmountX,
            dragAmountY,
            value
        );
        dragAmountX = fixedX;
        dragAmountY = fixedY;
    }
    function fixDragAmount(fixedX: number, fixedY: number, value: number) {
        if (fixedY > value) {
            fixedY = value;
        }
        if (fixedY < -value) {
            fixedY = -value;
        }
        if (fixedX > value) {
            fixedX = value;
        }
        if (fixedX < -value) {
            fixedX = -value;
        }

        return { fixedX, fixedY };
    }
    function calculateOffset(zoomValue: string) {
        return (
            (MAZE_SIZE * parseFloat(zoomValue) - VIEW_PORT_SIZE) /
            parseFloat(zoomValue) /
            2
        );
    }
    function handleKeyboard(e: KeyboardEvent) {
        switch (e.key) {
            case keyboardMenu:
                menuOpen = !menuOpen;
                break;
            case "w":
                dragAmountY = value;
                break;
            case "a":
                dragAmountX = value;
                break;
            case "s":
                dragAmountY = -value;
                break;
            case "d":
                dragAmountX = -value;
                break;
            case "c":
                dragAmountX = 0;
                dragAmountY = 0;
                break;
            case "z":
                dragAmountX = 83;
                break;
            default:
                break;
        }
    }
    function handleZoom(e: WheelEvent) {
        if (menuOpen || currentMaze.length == 0) return;
        let newZoom = (
            e.deltaY > 0
                ? Math.max(
                      parseFloat((parseFloat(zoom) - zoomAmount).toFixed(1)),
                      1.0
                  )
                : parseFloat((parseFloat(zoom) + zoomAmount).toFixed(1))
        ).toString();

        let newOffset = calculateOffset(newZoom);

        const { fixedX, fixedY } = fixDragAmount(
            dragAmountX,
            dragAmountY,
            newOffset
        );

        dragAmountX = fixedX;
        dragAmountY = fixedY;
        zoom = newZoom;
    }
</script> -->
