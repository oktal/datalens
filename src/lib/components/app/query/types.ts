export type QueryData = {
    queryString: string,
    queryAll: boolean,
    streamId?: string,
    queryError?: any,

    columns: string[],
    rows: string[][],

    rowsPerPage: number,
    currentPage: number,
}
