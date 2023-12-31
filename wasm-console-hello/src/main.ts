import './style.css'
import init from '../wasm/pkg'

document.querySelector<HTMLDivElement>('#app')!.innerHTML = `
  <div>
    <h1>Rust + WASM</h1>
    <div class="card">
      View Console
    </div>
  </div>
`

window.addEventListener('load', async () => {
  await init()
})
