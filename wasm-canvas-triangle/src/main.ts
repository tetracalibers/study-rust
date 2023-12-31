import './style.css'
import init from '../wasm/pkg'

window.addEventListener('load', async () => {
  await init()
})
