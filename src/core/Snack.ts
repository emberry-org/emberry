export interface Snack {
    title: string;
    description: string;

    actions?: SnackAction[];
}

export interface SnackAction {
    label: string;
    class?: string;

    // These are related to the event / command to be emitted.

    /** The (key / name) of the (event / command) to emit */
    key: string;
    /** Whether the given key is that of an event or a command */
    isCommand?: boolean;
    /** The optional payload to send along with the (event / command) */
    payload?: any;
}