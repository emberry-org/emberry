export interface Snack {
    title: string;
    description: string;

    actions: SnackAction[];
}

export interface SnackAction {
    label: string;
    class?: string;
    handler: () => void;
}