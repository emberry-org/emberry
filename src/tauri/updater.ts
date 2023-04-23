import { onCleanup } from "solid-js";
import { checkUpdate, onUpdaterEvent } from "@tauri-apps/api/updater";
import { setItem } from "../lib/storage";

/**
 * Setup the tauri auto updater.
 */
export async function setupUpdater() {
    const unlisten = await onUpdaterEvent(({ error, status }) => {
        setItem(sessionStorage, "tauri-updater", JSON.stringify({
            error,
            status,
            manifest: undefined
        }));
    }); // TAURI

    try {
        const { shouldUpdate, manifest } = await checkUpdate() // TAURI

        if (shouldUpdate) {
            // You could show a dialog asking the user if they want to install the update here.
            console.log(
                `Installing update ${manifest?.version}, ${manifest?.date}, ${manifest?.body}`
            )

            setItem(sessionStorage, "tauri-updater", JSON.stringify({
                error: undefined,
                status: "UPDATE",
                manifest
            }));
        }
    } catch (error) {
        console.error(error)
    }

    onCleanup(() => { 
        unlisten(); // TAURI
    });
}