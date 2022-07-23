import type { VoidAction } from "./cmd-center/Cmd";

export interface Snack {
    title: string;
    description: string;

    actions: SnackAction[];
}

export interface SnackAction {
    label: string;
    class?: string;
    handler: VoidAction;
}