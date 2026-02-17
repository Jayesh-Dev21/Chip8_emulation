import init, * as wasm from "./pkg/wasm.js"

const WIDTH = 64
const HEIGHT = 32
const SCALE = 15
let TICKS_PER_FRAME = 10
let anim_frame = 0

const canvas = document.getElementById("canvas")
canvas.width = WIDTH * SCALE
canvas.height = HEIGHT * SCALE

const ctx = canvas.getContext("2d")
ctx.fillStyle = "black"
ctx.fillRect(0, 0, WIDTH * SCALE, HEIGHT * SCALE)

const input = document.getElementById("fileinput")

async function run() {
    await init()
    let chip8 = new wasm.EmuWasm()

    document.addEventListener("keydown", function(evt) {
        chip8.keypress(evt, true)
    })

    document.addEventListener("keyup", function(evt) {
        chip8.keypress(evt, false)
    })

    const keypad = document.querySelector(".keypad")
    if (keypad) {
        const buttons = keypad.querySelectorAll("button");
        buttons.forEach(button => {
            button.addEventListener("pointerdown", function(evt) {
                evt.preventDefault(); // Prevent default behavior
                const key = this.dataset.key;
                if (key) {
                    chip8.virtual_keypress(key, true);
                }
            });

            button.addEventListener("pointerup", function(evt) {
                evt.preventDefault(); // Prevent default behavior
                const key = this.dataset.key;
                if (key) {
                    chip8.virtual_keypress(key, false);
                }
            });
        });
    }

    input.addEventListener("change", function(evt) {
        // Stop previous game from rendering, if one exists
        if (anim_frame != 0) {
            window.cancelAnimationFrame(anim_frame)
        }

        let file = evt.target.files[0]
        if (!file) {
            alert("Failed to read file")
            return
        }

        // Load in game as Uint8Array, send to .wasm, start main loop
        let fr = new FileReader()
        fr.onload = function(e) {
            let buffer = fr.result
            const rom = new Uint8Array(buffer)
            chip8.reset()
            chip8.load_game(rom)
            mainloop(chip8)
        }
        fr.readAsArrayBuffer(file)
    }, false)

    const romSelect = document.getElementById("rom-select")
    romSelect.addEventListener("change", async function(evt) {
        if (anim_frame != 0) {
            window.cancelAnimationFrame(anim_frame)
        }

        const romName = evt.target.value
        console.log("Selected ROM:", romName)
        if (!romName) {
            return // No ROM selected
        }

        try {
            const response = await fetch(`/c8games/${romName}`)
            console.log("Fetch response:", response)
            if (!response.ok) {
                throw new Error(`HTTP error! status: ${response.status}`)
            }
            const rom = new Uint8Array(await response.arrayBuffer())
            chip8.reset()
            chip8.load_game(rom)
            mainloop(chip8)
            romSelect.blur() // Remove focus from the dropdown
        } catch (error) {
            console.error("Error loading ROM:", error)
            alert("Failed to load ROM: " + romName)
        }
    })

    const speedSelect = document.getElementById("speed-select")
    speedSelect.addEventListener("change", function(evt) {
        const speedMultiplier = parseFloat(evt.target.value)
        TICKS_PER_FRAME = Math.round(10  * speedMultiplier)
        console.log("New TICKS_PER_FRAME:", TICKS_PER_FRAME)
        // Remove focus from the speed selector immediately
        speedSelect.blur()
    })
    
    // Prevent key events from changing the speed selector after it loses focus
    speedSelect.addEventListener("blur", function(evt) {
        // Add a flag to prevent re-focusing
        speedSelect.setAttribute("data-blurred", "true")
    })
    
    speedSelect.addEventListener("focus", function(evt) {
        // Remove the flag when user intentionally focuses
        speedSelect.removeAttribute("data-blurred")
    })
    
    // Prevent number keys from changing the speed selector when it should be blurred
    speedSelect.addEventListener("keydown", function(evt) {
        if (speedSelect.getAttribute("data-blurred") === "true") {
            evt.preventDefault()
            evt.stopPropagation()
            speedSelect.blur()
        }
    })
}

function mainloop(chip8) {
    // Only draw every few ticks
    for (let i = 0; i < TICKS_PER_FRAME; i++) {
        chip8.tick()
    }
    chip8.tick_timers()

    // Clear the canvas before drawing
    ctx.fillStyle = "black"
    ctx.fillRect(0, 0, WIDTH * SCALE, HEIGHT * SCALE)
    // Set the draw color back to white before we render our frame
    ctx.fillStyle = "white"
    chip8.draw_screen(SCALE)

    anim_frame = window.requestAnimationFrame(() => {
        mainloop(chip8)
    })
}

run().catch(console.error)