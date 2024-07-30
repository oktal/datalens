import Dialog from '@smui/dialog'

export class DialogHandle<T> {
    dialog?: Dialog;
    value: T;

    beforeAccept?: (value: T) => boolean;
    reset?: (value: T) => void;

    resolve_?: (value: T) => void;
    reject_?: (reason?: any) => void;

    constructor(value: T) {
        this.value = value;
    }

    show(): Promise<T> {
        return new Promise<T>((resolve, reject) => {
            this.resolve_ = resolve;
            this.reject_ = reject;

            this.open();
        });
    }

    accept() {
        if (this.beforeAccept) {
            if (!this.beforeAccept(this.value))
                return;
        }

        this.close();

        if (this.resolve_) {
            const value = structuredClone(this.value);
            if (this.reset)
                this.reset(this.value);
            this.resolve_(value);

        }
    }

    cancel() {
        this.close();
        if (this.reject_ != undefined) this.reject_('action has been cancelled by user');
    }

    handleAction(e: CustomEvent<{ action: string }>) {
        if (e.detail.action == 'accept') this.accept();
        else this.cancel();
    }

    open() {
        this.dialog?.setOpen(true);
    }

    close() {
        this.dialog?.setOpen(false);
    }
}

export function createHandle<T>(value: T): DialogHandle<T> {
    return new DialogHandle<T>(value);
}
