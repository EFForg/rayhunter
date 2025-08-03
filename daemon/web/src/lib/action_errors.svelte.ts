export class ActionError extends Error {
    // The number of this an identical error has happened.
    // This is shown as a number next to the error in the UI.
    times = $state(1);

    constructor(message: string, cause: Error) {
        super(message);
        this.cause = cause;
    }
}

export const action_errors: ActionError[] = $state([]);

export function add_error(e: Error, msg: string): void {
    for (const existing of action_errors) {
        if (existing.message === msg) {
            existing.times += 1;
            return;
        }
    }
    const action_error = new ActionError(msg, e);
    action_errors.unshift(action_error);
    console.log(action_errors.length);
}
