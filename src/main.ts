import setupOS from '@core/OppSys'
import App from './App.svelte'

setupOS();

const app = new App({
  target: document.getElementById('app')
})

export default app
