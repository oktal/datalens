export type QueryData = {
    queryString: string,
    queryAll: boolean,
    streamId?: string,
    queryError?: string,

    columns: string[],
    rows: string[][],

    rowsPerPage: number,
    currentPage: number,
}
