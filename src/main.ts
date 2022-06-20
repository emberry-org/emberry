import setupOS from '@core/OppSys';
import { setupPersistentStore } from '@store';
import App from './App.svelte';

setupPersistentStore();
setupOS();

const app = new App({
  target: document.getElementById('app')
})

export default app
