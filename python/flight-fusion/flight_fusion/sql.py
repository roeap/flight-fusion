from sqlalchemy.dialects.postgresql import (
    BIGINT,
    BOOLEAN,
    DATE,
    FLOAT,
    INTEGER,
    REAL,
    SMALLINT,
    TIME,
    TIMESTAMP,
    VARCHAR,
)

type_map = {
    "Utf8": VARCHAR,
    "Float32": FLOAT,
    "Int16": SMALLINT,
    "Int32": INTEGER,
    "Int64": BIGINT,
    "Float64": REAL,
    "Boolean": BOOLEAN,
    "Date32": DATE,
    "Time64": TIME,
    "Timestamp": TIMESTAMP,
}
