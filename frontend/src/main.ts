import './app.css';
import { mount } from 'svelte';
import App from './App.svelte';

const target = document.getElementById('app');
if (!target) {
  throw new Error('Mount node #app is missing from index.html');
}

const app = mount(App, { target });

export default app;
