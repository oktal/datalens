import { invoke } from "@tauri-apps/api";

import type { DatabaseModel, Row, StreamId } from "./types";

export async function sql(query: string) {
    const res = await invoke("sql", {
        query
    });

    return res;
}

export async function streamCreate(query: string): Promise<StreamId> {
    const res = await invoke<StreamId>("sql_stream", {
        query
    });

    return res;
}

export async function streamFetch(stream: StreamId): Promise<Row[]> {
    const res = await invoke<Row[]>("stream_next", {
        streamId: stream
    });

    return res;
}

export async function listDatabases(): Promise<DatabaseModel[]> {
    const res = await invoke<DatabaseModel[]>("list_databases");
    return res;
}
