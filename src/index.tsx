/* @refresh reload */
import { Route, Router, Routes } from '@solidjs/router';
import { Component } from 'solid-js';
import { render } from 'solid-js/web';

import './index.css';

const root = document.getElementById('root');

if (import.meta.env.DEV && !(root instanceof HTMLElement)) {
    throw new Error(
        'Root element not found.',
    );
}

/* --- app root component --- */
import StatusBar from "./lib/statusbar";
import FriendsList from "./lib/panels/friends";
// const Home = lazy(() => import("./tree/home"));
// const Room = lazy(() => import("./tree/room"));
import Home from "./pages/home";
import Room from "./pages/room";

const App: Component = () => {
    return (
        <>
            <main class="content">
                <FriendsList />
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
