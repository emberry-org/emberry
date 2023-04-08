/* @refresh reload */
import { Route, Router, Routes } from '@solidjs/router';
import { Component, lazy } from 'solid-js';
import { render } from 'solid-js/web';

import './index.css';

const root = document.getElementById('root');

if (import.meta.env.DEV && !(root instanceof HTMLElement)) {
    throw new Error(
        'Root element not found.',
    );
}

/* --- app root component --- */
import StatusBar from './lib/statusbar';
const Home = lazy(() => import("./tree/home"));
const Room = lazy(() => import("./tree/room"));

const App: Component = () => {
    return (
        <>
            <h1>Emberry</h1>
            <main class="content">
                <Routes>
                    <Route path="/" component={Home} />
                    <Route path="/room/:id" component={Room} />

                    {/* fallback route */}
                    <Route path="*" element={<p>Page not found inside the tree. <a href="/">return</a></p>} />
                </Routes>
            </main>
            <StatusBar />
        </>
    );
};

/* --- app router --- */
render(() => (
    <Router>
        <App />
    </Router>
), root!);

/* --- tauri setup --- */
import initTauri from './tauri';
initTauri();
