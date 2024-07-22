import { invoke } from "@tauri-apps/api";

import { type DatabaseModel } from "./types";

export async function sql(query: string) {
    const res = await invoke("sql", {
        query
    });

    return res;
}

export async function listDatabases(): Promise<DatabaseModel[]> {
    const res = await invoke<DatabaseModel[]>("list_databases");
    return res;
}
