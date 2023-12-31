import './style.css'
import init, { greet } from '../wasm/pkg'

document.querySelector<HTMLDivElement>('#app')!.innerHTML = `
  <div>
    <h1>Rust + WASM</h1>
    <div class="card">
      <button id="trigger" type="button">Greet</button>
    </div>
  </div>
`

window.addEventListener('load', async () => {
  await init()
})

document
  .querySelector<HTMLButtonElement>('#trigger')!
  .addEventListener('click', () => {
    greet()
  })
