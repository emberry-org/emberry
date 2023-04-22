import { Component, Show, createSignal } from "solid-js";
import { setItem, getItem, onItem } from "./storage";
import { listen } from "@tauri-apps/api/event";

import Connected from '@ico/rhizome/conn.svg?component-solid';
import Disconnected from '@ico/rhizome/discon.svg?component-solid';
import "./rhizome.css";

type RhizomeStatus = "connecting" | "connected" | "disconnected" | "failed";

/** Rhizome status. */
interface Rhizome {
    status: RhizomeStatus,
    contime: string | undefined
}

/**
 * Update the rhizome status in storage.
 * @param status The new status.
 * @param contime The connection time.
 */
function updateStatus(status: RhizomeStatus, contime: string) {
    setItem(sessionStorage, "rhizome-status", JSON.stringify({
        status,
        contime
    }));
}

const statusIndicator: Component = () => {
    /** The current status of the rhizome connection */
    const [rhizome, setStatus] = createSignal<Rhizome>(
        JSON.parse(getItem(sessionStorage, "rhizome-status")) ?? { status: "connecting", contime: undefined }
    );

    // Subscribe to rhizome status events:
    listen("rz-con", (evt) => updateStatus("connected", evt.payload as string));    // TAURI
    listen("rz-dc", (evt) => updateStatus("disconnected", evt.payload as string));  // TAURI
    listen("rz-f", (evt) => updateStatus("failed", evt.payload as string));         // TAURI

    // Receive updates on the rhizome status.
    onItem("rhizome-status", setStatus);

    return (
        <div class="rhizome-status" title={(rhizome().contime ?? "0") + "ms"} data-status={rhizome().status}>
            <Show
                when={rhizome().status == "connected"}
                fallback={<Disconnected width="20px" />}
            >
                <Connected width="20px" />
            </Show>
        </div>
    );
};

export default statusIndicator;