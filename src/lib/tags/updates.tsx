import { Component, Show, createSignal } from "solid-js";
import { relaunch } from "@tauri-apps/api/process";
import { installUpdate } from "@tauri-apps/api/updater";
import { getItem, onItem } from "../storage";

import Download from '@ico/download.svg?component-solid';
import "./updates.css";

type UpdaterStatus = "ERROR" | "UNKNOWN" | "UPDATE";

/** Updater status. */
interface Updater {
    status: UpdaterStatus,
    error?: string,
    manifest?: {
        version: string,
        date: string,
        body: string
    }
}

/** Update the client. */
async function update() {
    // Install the update. This will also restart the app on Windows!
    await installUpdate() // TAURI

    // On macOS and Linux you will need to restart the app manually.
    // You could use this step to display another confirmation dialog.
    await relaunch() // TAURI
}

const updateIndicator: Component = () => {
    /** The current updater status */
    const [update, setStatus] = createSignal<Updater>(
        JSON.parse(getItem(sessionStorage, "tauri-updater")) ?? { status: "UNKNOWN", error: undefined }
    );

    // Receive updates on the updater status.
    onItem("tauri-updater", setStatus);

    return (
        <button class="updater-status" onMouseDown={update} data-status={update().status}>
            <Show
                when={update().status === "UPDATE"}
            >
                <Download width="20px" stroke-width="1px" stroke="currentColor" />
            </Show>
            <span>v0.0.0</span>
        </button>
    );
};

export default updateIndicator;