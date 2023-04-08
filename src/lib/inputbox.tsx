import { createCodeMirror, createEditorControlledValue, createLazyCompartmentExtension } from "solid-codemirror";
import { EditorView, keymap } from "@codemirror/view";
import { Component, createSignal } from "solid-js";
import { emit } from "@tauri-apps/api/event";
import { updateItem } from "./storage";

import "./inputbox.css";

const inputBox: Component<{room_id: string}> = (props) => {
    const [content, setContent] = createSignal("");
    const { editorView, ref: editorRef, createExtension } = createCodeMirror({
        onValueChange: setContent,
    });
    createEditorControlledValue(editorView, content);

    /* Editor theme */
    const theme = EditorView.theme({
        "&": {
            color: "#fff",
            backgroundColor: "var(--bg-100)"
        },
    }, { dark: true });

    /* Editor shortcuts keymap */
    const shortcuts = keymap.of([
        {
            /* Send message on enter */
            key: "Enter", 
            run: () => { 
                emit(`send_message_${props.room_id}`, { Chat: content().trim() }); // TAURI

                // Insert our message into the messages for the current room.
                updateItem(sessionStorage, `messages-${props.room_id}`, (chat: { origin: string, content: string }[]) => {
                    if (!chat) chat = [];
                    chat.push({
                        origin: "Local",
                        content: content().trim(),
                    });
                    return chat;
                });

                console.log("send msg:\n" + content().trim()); 
                setContent(() => ""); 
                return true; 
            } 
        }
    ]);

    /* Lazy load the slash command autocompletion (58kB) */
    createLazyCompartmentExtension(
        () => import('./inputbox/slash').then(res => res.default),
        editorView
    );

    createExtension(theme);
    createExtension(shortcuts);

    return <div ref={editorRef} />;
}

export default inputBox;
