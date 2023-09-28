import './app.css'
import Table from "./lib/Table.svelte"

const app = new Table({
  target: document.getElementById('app'),
})

export default app
