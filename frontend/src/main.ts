import './app.css';
import App from './App.svelte';

const target = document.getElementById('app');
if (!target) {
  throw new Error('Mount node #app is missing from index.html');
}

const app = new App({ target });

export default app;
