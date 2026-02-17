import init, * as wasm from "./pkg/wasm.js"


/* CONFIG */

const WIDTH = 64
const HEIGHT = 32
const SCALE = 10

let TICKS = 10
let running = false
let frame = 0


/* ELEMENTS */

const canvas = document.getElementById("canvas")
const status = document.getElementById("status")
const dropdown = document.getElementById("rom-dropdown")
const button = document.getElementById("start-stop-button")

canvas.width = WIDTH * SCALE
canvas.height = HEIGHT * SCALE

const ctx = canvas.getContext("2d")


/* AUDIO */

let audioCtx = null
let osc = null

function startBeep() {

  if (osc) return

  audioCtx ??= new AudioContext()

  osc = audioCtx.createOscillator()
  osc.type = "square"

  osc.connect(audioCtx.destination)
  osc.start()
}

function stopBeep() {

  if (!osc) return

  osc.stop()
  osc.disconnect()

  osc = null
}


/* MAIN */

let chip8 = null


async function boot() {

  status.textContent = "Loading WASM..."

  await init()

  chip8 = new wasm.EmuWasm()

  status.textContent = "Ready"

  setupKeyboard()
  setupMenu()
}


function setupKeyboard() {

  document.addEventListener("keydown", e => {

    chip8.virtual_keypress(
      e.key.toLowerCase(),
      true
    )

    audioCtx?.resume()
  })


  document.addEventListener("keyup", e => {

    chip8.virtual_keypress(
      e.key.toLowerCase(),
      false
    )
  })
}


/* ROM MENU */

function setupMenu() {

  dropdown.onchange = () => {

    if (dropdown.value !== "SELECT ROM") {
      button.disabled = false
    }
  }


  button.onclick = () => {

    if (running) {
      stop()
    } else {
      start()
    }
  }
}


/* START */

async function start() {

  const rom = dropdown.value

  if (!rom) return


  status.textContent = "Loading ROM..."


  const res = await fetch(`/c8games/${rom}`)

  const data = new Uint8Array(
    await res.arrayBuffer()
  )


  chip8.reset()
  chip8.load_game(data)


  running = true
  button.textContent = "STOP"
  status.textContent = "Running"

  loop()
}


/* STOP */

function stop() {

  running = false

  cancelAnimationFrame(frame)

  button.textContent = "START"
  status.textContent = "Stopped"

  stopBeep()
}


/* LOOP */

function loop() {

  if (!running) return


  for (let i = 0; i < TICKS; i++) {
    chip8.tick()
  }

  chip8.tick_timers()


  /* SOUND */

  if (chip8.get_sound_timer() > 0) {
    startBeep()
  } else {
    stopBeep()
  }


  /* DRAW */

  ctx.fillStyle = "black"
  ctx.fillRect(0, 0, canvas.width, canvas.height)

  ctx.fillStyle = "white"

  chip8.draw_screen(SCALE)


  frame = requestAnimationFrame(loop)
}


/* BOOT */

boot()
