export type DatabaseModel = {
    name: string,
    schemas: {
        name: string
        tables: {
            name: string,
            schema: {
                fields: {
                    name: string,
                    data_type:
                    "Null" | "Boolean"
                    | "Int8" | "Int16" | "Int32" | "Int64"
                    | "UInt8" | "UInt16" | "UInt32" | "UInt64"
                    | "Float32" | "Float64"
                    | "Decimal128" | "Decimal256"
                    | "Date32" | "Date64"
                    | "Time32" | "Time64" | { Timestamp: [TimeUnit, TimeZone] }
                    | "LargeUtf8" | "Utf8",
                    nullable: boolean,
                    metadata: Record<string, string>
                }[],
                metadata: Record<string, string>,
            }
        }[]
    }[]
}

export type StreamId = string;

export type Row = {
    columns: string[],
    values: string[]
}

export type Database = {
    name: string,
    schemas: {
        name: string
        tables: {
            name: string,
            schema: {
                fields: {
                    name: string,
                    data_type: DataType,
                    nullable: boolean,
                    metadata: Record<string, string>
                }[],
                metadata: Record<string, string>,
            }
        }
    }[]
}

export function toDatabase(db: DatabaseModel): Database {
    const database: Database = {
        name: db.name,
        schemas: db.schemas.map((schemaModel: any) => {
            const schema = {
                name: schemaModel.name,
                tables: schemaModel.tables.map((tableModel: any) => {
                    const tableSchema = {
                        fields: tableModel.schema.fields.map(
                            (fieldModel: any) => {
                                let data_type = toDataType(
                                    fieldModel.data_type,
                                );

                                const field = {
                                    name: fieldModel.name,
                                    data_type: data_type!,
                                    nullable: fieldModel.nullable,
                                    metadata: fieldModel.metadata,
                                };

                                return field;
                            },
                        ),
                        metadata: tableModel.schema.metadta,
                    };

                    const table = {
                        name: tableModel.name,
                        schema: tableSchema,
                    };

                    return table;
                }),
            };

            return schema;
        }),
    };

    return database;;
}

export type Null = {
    kind: "null",
    logical: "null"
};

export type Boolean = {
    kind: "boolean",
    logical: "boolean"
}

export type Int8 = {
    kind: "int8",
    logical: "integer"
}

export type Int16 = {
    kind: "int16",
    logical: "integer"
}

export type Int32 = {
    kind: "int32",
    logical: "integer"
}

export type Int64 = {
    kind: "int64",
    logical: "integer"
}

export type UInt8 = {
    kind: "uint8",
    logical: "integer"
}

export type UInt16 = {
    kind: "uint16",
    logical: "integer"
}

export type UInt32 = {
    kind: "uint32",
    logical: "integer"
}

export type UInt64 = {
    kind: "uint64",
    logical: "integer"
}

export type Float32 = {
    kind: "float32",
    logical: "decimal"
}

export type Float64 = {
    kind: "float64",
    logical: "decimal"
}

export type Decimal128 = {
    kind: "decimal128",
    logical: "decimal",
    precision: number,
    scale: number,
}

export type Decimal256 = {
    kind: "decimal256",
    logical: "decimal",
    precision: number,
    scale: number,
}

export type Date32 = {
    kind: "date32",
    logical: "date"
}

export type Date64 = {
    kind: "date64",
    logical: "date"
}

export type Time32 = {
    kind: "time32",
    logical: "time"
    unit: TimeUnit
}

export type Time64 = {
    kind: "time64",
    logical: "time",
    unit: TimeUnit
}

export type Duration = {
    kind: "duration",
    logical: "time",
    unit: TimeUnit
}

export type Timestamp = {
    kind: "timestamp",
    logical: "timestamp",
    unit: TimeUnit,
    tz: TimeZone
};

export type LargeUtf8 = {
    kind: "largeutf8",
    logical: "string"
}

export type Utf8 = {
    kind: "utf8",
    logical: "string"
}

export type DataType =
    Null
    | Boolean
    | Int8
    | Int16
    | Int32
    | Int64
    | UInt8
    | UInt16
    | UInt32
    | UInt64
    | Float32
    | Float64
    | Decimal128
    | Decimal256
    | Date32
    | Date64
    | Time32
    | Time64
    | Duration
    | Timestamp
    | LargeUtf8
    | Utf8;

export type LogicalDataType = Extract<DataType, { logical: string }>["logical"];

export type TimeZone = string | undefined;

export type FileType = "csv" | "arrow" | "parquet" | "avro" | "json";
export type TimeUnit = "Second" | "Millisecond" | "Microsecond" | "Nanosecond";

function toTimeUnit(tu: any): TimeUnit | undefined {
    if (typeof tu === "string") {
        switch (tu.toLowerCase()) {
            case "second":
                return "Second";
            case "millisecond":
                return "Millisecond";
            case "microsecond":
                return "Microsecond";
            case "nanosecond":
                return "Nanosecond";
        }
    }

    return undefined;
}

function toTimezone(tz: any): TimeZone {
    if (typeof tz === "string") {
        return tz;
    }

    return undefined;
}

function toDataType(dt: any): DataType | undefined {
    if (typeof dt === "string") {
        const dataType = dt.toLowerCase();

        switch (dataType) {
            case "null":
                return {
                    kind: dataType,
                    logical: "null",
                };
            case "boolean":
                return {
                    kind: dataType,
                    logical: "boolean",
                };
            case "int8":
            case "int16":
            case "int32":
            case "int64":
            case "uint8":
            case "uint16":
            case "uint32":
            case "uint64":
                return {
                    kind: dataType,
                    logical: "integer",
                };
            case "float32":
            case "float64":
                return {
                    kind: dataType,
                    logical: "decimal",
                };
            case "date32":
            case "date64":
                return {
                    kind: dataType,
                    logical: "date",
                };
            case "utf8":
            case "largeutf8":
                return {
                    kind: dataType,
                    logical: "string",
                };
        }
    } else {
        if ("Timestamp" in dt) {
            const [unit, tz] = dt["Timestamp"];
            const timeUnit = toTimeUnit(unit);

            if (timeUnit === undefined) {
                return undefined;
            }

            return {
                kind: "timestamp",
                logical: "timestamp",
                unit: timeUnit,
                tz: toTimezone(tz),
            };
        }
    }

    return undefined;
}
