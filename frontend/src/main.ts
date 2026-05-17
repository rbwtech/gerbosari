import './app.css';
import { mount } from 'svelte';
import App from './App.svelte';

// Hash-router compat: if a visitor lands on a real path like /admin/login
// (e.g. from a bookmark or someone typing the URL directly), rewrite to the
// hash form so the in-app router picks it up. nginx already falls back to
// index.html for unknown paths, so we always reach this script.
if (typeof window !== 'undefined') {
  const { pathname, search, hash } = window.location;
  if (!hash && pathname && pathname !== '/' && pathname !== '/index.html') {
    history.replaceState(null, '', '/#' + pathname + search);
  }
}

const target = document.getElementById('app');
if (!target) {
  throw new Error('Mount node #app is missing from index.html');
}

const app = mount(App, { target });

export default app;
