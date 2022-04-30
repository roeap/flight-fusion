from __future__ import annotations

from typing import Any, AnyStr, Iterator

import pandas as pd

class RecordBatchReader: ...

class DataType:
    @property
    def id(self): ...
    @property
    def num_fields(self) -> int:
        """The number of child fields."""
        ...
    @property
    def num_buffers(self) -> int:
        """Number of data buffers required to construct Array type excluding children."""
        ...
    def equals(self, other: DataType | str) -> bool:
        """Return true if type is equivalent to passed value.

        Args:
            other : DataType or string convertible to DataType

        Returns:
            is_equal : bool
        """
    def to_pandas_dtype(self):
        """Return the equivalent NumPy / Pandas dtype."""
        ...

class Field:
    @property
    def name(self) -> str: ...
    @property
    def metadata(self) -> dict[str, str]: ...
    @property
    def nullable(self) -> bool: ...

class Schema:
    """
    A named collection of types a.k.a schema. A schema defines the
    column names and types in a record batch or table data structure.
    They also contain metadata about the columns. For example, schemas
    converted from Pandas contain metadata about their original Pandas
    types so they can be converted back to the same types.

    Warnings
    --------
    Do not call this class's constructor directly. Instead use
    :func:`pyarrow.schema` factory function which makes a new Arrow
    Schema object.

    Examples
    --------
    Create a new Arrow Schema object:
    >>> import pyarrow as pa
    >>> pa.schema([
    ...     ('some_int', pa.int32()),
    ...     ('some_string', pa.string())
    ... ])
    some_int: int32
    some_string: string
    Create Arrow Schema with metadata:
    >>> pa.schema([
    ...     pa.field('n_legs', pa.int64()),
    ...     pa.field('animals', pa.string())],
    ...     metadata={"n_legs": "Number of legs per animal"})
    n_legs: int64
    animals: string
    -- schema metadata --
    n_legs: 'Number of legs per animal'
    """

    def __len__(self) -> int: ...
    def __getitem__(self, key) -> Field: ...
    def __iter__(self) -> Iterator[Field]: ...
    @property
    def pandas_metadata(self) -> dict[str, Any]:
        """
        Return deserialized-from-JSON pandas metadata field (if it exists)
        Examples
        --------
        >>> import pyarrow as pa
        >>> import pandas as pd
        >>> df = pd.DataFrame({'n_legs': [2, 4, 5, 100],
        ...                    'animals': ["Flamingo", "Horse", "Brittle stars", "Centipede"]})
        >>> schema = pa.Table.from_pandas(df).schema
        Select pandas metadata field from Arrow Schema:
        >>> schema.pandas_metadata
        {'index_columns': [{'kind': 'range', 'name': None, 'start': 0, 'stop': 4, 'step': 1}], ...
        """
        ...
    @property
    def names(self) -> list[str]:
        """The schema's field names.

        Examples
        --------
        >>> import pyarrow as pa
        >>> schema = pa.schema([
        ...     pa.field('n_legs', pa.int64()),
        ...     pa.field('animals', pa.string())])
        Get the names of the schema's fields:
        >>> schema.names
        ['n_legs', 'animals']
        """
        ...
    @property
    def types(self) -> list[DataType]:
        """The schema's field types.

        Examples
        --------
        >>> import pyarrow as pa
        >>> schema = pa.schema([
        ...     pa.field('n_legs', pa.int64()),
        ...     pa.field('animals', pa.string())])
        Get the types of the schema's fields:
        >>> schema.types
        [DataType(int64), DataType(string)]
        """
        ...
    @property
    def metadata(self) -> dict[bytes, bytes]:
        """The schema's metadata.

        Examples
        --------
        >>> import pyarrow as pa
        >>> schema = pa.schema([
        ...     pa.field('n_legs', pa.int64()),
        ...     pa.field('animals', pa.string())],
        ...     metadata={"n_legs": "Number of legs per animal"})
        Get the metadata of the schema's fields:
        >>> schema.metadata
        {b'n_legs': b'Number of legs per animal'}
        """
        ...
    def empty_table(self) -> Table:
        """
        Provide an empty table according to the schema.

        Examples
        --------
        >>> import pyarrow as pa
        >>> schema = pa.schema([
        ...     pa.field('n_legs', pa.int64()),
        ...     pa.field('animals', pa.string())])
        Create an empty table with schema's fields:
        >>> schema.empty_table()
        pyarrow.Table
        n_legs: int64
        animals: string
        ----
        n_legs: [[]]
        animals: [[]]
        """
        ...
    @classmethod
    def from_pandas(cls, df: pd.DataFrame, preserve_index: bool | None = None):
        """
        Returns implied schema from dataframe

        Args:
            df: pandas.DataFrame
            preserve_index : bool, default True
                Whether to store the index as an additional column (or columns, for
                MultiIndex) in the resulting `Table`.
                The default of None will store the index as a column, except for
                RangeIndex which is stored as metadata only. Use
                ``preserve_index=True`` to force it to be stored as a column.
        Returns:
            pyarrow.Schema

        Examples:
            >>> import pandas as pd
            >>> import pyarrow as pa
            >>> df = pd.DataFrame({
            ...     'int': [1, 2],
            ...     'str': ['a', 'b']
            ... })
            Create an Arrow Schema from the schema of a pandas dataframe:
            >>> pa.Schema.from_pandas(df)
            int: int64
            str: string
            -- schema metadata --
            pandas: '{"index_columns": [{"kind": "range", "name": null, ...
        """
        ...
    def field(self, i: int | str) -> Field:
        """Select a field by its column name or numeric index.
        Args:
            i : int or string

        Returns:
            pyarrow.Field

        Examples
        --------
        >>> import pyarrow as pa
        >>> schema = pa.schema([
        ...     pa.field('n_legs', pa.int64()),
        ...     pa.field('animals', pa.string())])
        Select the second field:
        >>> schema.field(1)
        pyarrow.Field<animals: string>
        Select the field of the column named 'n_legs':
        >>> schema.field('n_legs')
        pyarrow.Field<n_legs: int64>
        """
        ...
    def get_field_index(self, name) -> int:
        """Return index of the unique field with the given name.

        Args:
            name: The name of the field to look up.

        Returns:
            index: The index of the field with the given name; -1 if the
                name isn't found or there are several fields with the given
                name.

        Examples
        --------
        >>> import pyarrow as pa
        >>> schema = pa.schema([
        ...     pa.field('n_legs', pa.int64()),
        ...     pa.field('animals', pa.string())])
        Get the index of the field named 'animals':
        >>> schema.get_field_index("animals")
        1
        Index in case of several fields with the given name:
        >>> schema = pa.schema([
        ...     pa.field('n_legs', pa.int64()),
        ...     pa.field('animals', pa.string()),
        ...     pa.field('animals', pa.bool_())],
        ...     metadata={"n_legs": "Number of legs per animal"})
        >>> schema.get_field_index("animals")
        -1
        """
        ...
    def with_metadata(self, metadata: dict[AnyStr, AnyStr]) -> Schema:
        """Add metadata as dict of string keys and values to Schema.

        Args:
            metadata: Keys and values must be string-like / coercible to bytes

        Returns:
            schema : pyarrow.Schema

        Examples
        --------
        >>> import pyarrow as pa
        >>> schema = pa.schema([
        ...     pa.field('n_legs', pa.int64()),
        ...     pa.field('animals', pa.string())])
        Add metadata to existing schema field:
        >>> schema.with_metadata({"n_legs": "Number of legs per animal"})
        n_legs: int64
        animals: string
        -- schema metadata --
        n_legs: 'Number of legs per animal'
        """
        ...

class RecordBatch:
    """
    Batch of rows of columns of equal length
    Warnings
    --------
    Do not call this class's constructor directly, use one of the
    ``RecordBatch.from_*`` functions instead.
    Examples
    --------
    >>> import pyarrow as pa
    >>> n_legs = pa.array([2, 2, 4, 4, 5, 100])
    >>> animals = pa.array(["Flamingo", "Parrot", "Dog", "Horse", "Brittle stars", "Centipede"])
    >>> names = ["n_legs", "animals"]
    Constructing a RecordBatch from arrays:
    >>> pa.RecordBatch.from_arrays([n_legs, animals], names=names)
    pyarrow.RecordBatch
    n_legs: int64
    animals: string
    >>> pa.RecordBatch.from_arrays([n_legs, animals], names=names).to_pandas()
       n_legs        animals
    0       2       Flamingo
    1       2         Parrot
    2       4            Dog
    3       4          Horse
    4       5  Brittle stars
    5     100      Centipede
    Constructing a RecordBatch from pandas DataFrame:
    >>> import pandas as pd
    >>> df = pd.DataFrame({'year': [2020, 2022, 2021, 2022],
    ...                    'month': [3, 5, 7, 9],
    ...                    'day': [1, 5, 9, 13],
    ...                    'n_legs': [2, 4, 5, 100],
    ...                    'animals': ["Flamingo", "Horse", "Brittle stars", "Centipede"]})
    >>> pa.RecordBatch.from_pandas(df)
    pyarrow.RecordBatch
    year: int64
    month: int64
    day: int64
    n_legs: int64
    animals: string
    >>> pa.RecordBatch.from_pandas(df).to_pandas()
       year  month  day  n_legs        animals
    0  2020      3    1       2       Flamingo
    1  2022      5    5       4          Horse
    2  2021      7    9       5  Brittle stars
    3  2022      9   13     100      Centipede
    Constructing a RecordBatch from pylist:
    >>> pylist = [{'n_legs': 2, 'animals': 'Flamingo'},
    ...           {'n_legs': 4, 'animals': 'Dog'}]
    >>> pa.RecordBatch.from_pylist(pylist).to_pandas()
       n_legs   animals
    0       2  Flamingo
    1       4       Dog
    You can also construct a RecordBatch using :func:`pyarrow.record_batch`:
    >>> pa.record_batch([n_legs, animals], names=names).to_pandas()
       n_legs        animals
    0       2       Flamingo
    1       2         Parrot
    2       4            Dog
    3       4          Horse
    4       5  Brittle stars
    5     100      Centipede
    >>> pa.record_batch(df)
    pyarrow.RecordBatch
    year: int64
    month: int64
    day: int64
    n_legs: int64
    animals: string
    """

    def replace_schema_metadata(self, metadata: dict[AnyStr, AnyStr] | None = None) -> RecordBatch:
        """
        Create shallow copy of record batch by replacing schema
        key-value metadata with the indicated new metadata (which may be None,
        which deletes any existing metadata

        Parameters
        ----------
        metadata : dict, default None

        Returns
        -------
        shallow_copy : RecordBatch

        Examples
        --------
        >>> import pyarrow as pa
        >>> n_legs = pa.array([2, 2, 4, 4, 5, 100])
        Constructing a RecordBatch with schema and metadata:
        >>> my_schema = pa.schema([
        ...     pa.field('n_legs', pa.int64())],
        ...     metadata={"n_legs": "Number of legs per animal"})
        >>> batch = pa.RecordBatch.from_arrays([n_legs], schema=my_schema)
        >>> batch.schema
        n_legs: int64
        -- schema metadata --
        n_legs: 'Number of legs per animal'
        Shallow copy of a RecordBatch with deleted schema metadata:
        >>> batch.replace_schema_metadata().schema
        n_legs: int64
        """
        ...
    @property
    def num_columns(self) -> int:
        """Number of columns

        Examples
        --------
        >>> import pyarrow as pa
        >>> n_legs = pa.array([2, 2, 4, 4, 5, 100])
        >>> animals = pa.array(["Flamingo", "Parrot", "Dog", "Horse", "Brittle stars", "Centipede"])
        >>> batch = pa.RecordBatch.from_arrays([n_legs, animals],
        ...                                     names=["n_legs", "animals"])
        >>> batch.num_columns
        2
        """
        ...
    @property
    def num_rows(self) -> int:
        """NNumber of rows

        Due to the definition of a RecordBatch, all columns have the same
        number of rows.

        Examples
        --------
        >>> import pyarrow as pa
        >>> n_legs = pa.array([2, 2, 4, 4, 5, 100])
        >>> animals = pa.array(["Flamingo", "Parrot", "Dog", "Horse", "Brittle stars", "Centipede"])
        >>> batch = pa.RecordBatch.from_arrays([n_legs, animals],
        ...                                     names=["n_legs", "animals"])
        >>> batch.num_rows
        6
        """
    @property
    def schema(self) -> Schema:
        """Schema of the RecordBatch and its columns

        Examples
        --------
        >>> import pyarrow as pa
        >>> n_legs = pa.array([2, 2, 4, 4, 5, 100])
        >>> animals = pa.array(["Flamingo", "Parrot", "Dog", "Horse", "Brittle stars", "Centipede"])
        >>> batch = pa.RecordBatch.from_arrays([n_legs, animals],
        ...                                     names=["n_legs", "animals"])
        >>> batch.schema
        n_legs: int64
        animals: string
        """
        ...
    @property
    def nbytes(self) -> int:
        """Total number of bytes consumed by the elements of the record batch.

        In other words, the sum of bytes from all buffer ranges referenced.
        Unlike `get_total_buffer_size` this method will account for array
        offsets.
        If buffers are shared between arrays then the shared
        portion will only be counted multiple times.
        The dictionary of dictionary arrays will always be counted in their
        entirety even if the array only references a portion of the dictionary.

        Examples
        --------
        >>> import pyarrow as pa
        >>> n_legs = pa.array([2, 2, 4, 4, 5, 100])
        >>> animals = pa.array(["Flamingo", "Parrot", "Dog", "Horse", "Brittle stars", "Centipede"])
        >>> batch = pa.RecordBatch.from_arrays([n_legs, animals],
        ...                                     names=["n_legs", "animals"])
        >>> batch.nbytes
        116
        """
        ...
    def field(self, i: int | str) -> Field:
        """
        Select a schema field by its column name or numeric index
        Parameters
        ----------
        i : The index or name of the field to retrieve

        Examples
        --------
        >>> import pyarrow as pa
        >>> n_legs = pa.array([2, 2, 4, 4, 5, 100])
        >>> animals = pa.array(["Flamingo", "Parrot", "Dog", "Horse", "Brittle stars", "Centipede"])
        >>> batch = pa.RecordBatch.from_arrays([n_legs, animals],
        ...                                     names=["n_legs", "animals"])
        >>> batch.field(0)
        pyarrow.Field<n_legs: int64>
        >>> batch.field(1)
        pyarrow.Field<animals: string>
        """

class Table:
    """
    A collection of top-level named, equal length Arrow arrays.
    Warnings
    --------
    Do not call this class's constructor directly, use one of the ``from_*``
    methods instead.
    Examples
    --------
    >>> import pyarrow as pa
    >>> n_legs = pa.array([2, 4, 5, 100])
    >>> animals = pa.array(["Flamingo", "Horse", "Brittle stars", "Centipede"])
    >>> names = ["n_legs", "animals"]
    Construct a Table from arrays:
    >>> pa.Table.from_arrays([n_legs, animals], names=names)
    pyarrow.Table
    n_legs: int64
    animals: string
    ----
    n_legs: [[2,4,5,100]]
    animals: [["Flamingo","Horse","Brittle stars","Centipede"]]
    Construct a Table from a RecordBatch:
    >>> batch = pa.record_batch([n_legs, animals], names=names)
    >>> pa.Table.from_batches([batch])
    pyarrow.Table
    n_legs: int64
    animals: string
    ----
    n_legs: [[2,4,5,100]]
    animals: [["Flamingo","Horse","Brittle stars","Centipede"]]
    Construct a Table from pandas DataFrame:
    >>> import pandas as pd
    >>> df = pd.DataFrame({'year': [2020, 2022, 2019, 2021],
    ...                    'n_legs': [2, 4, 5, 100],
    ...                    'animals': ["Flamingo", "Horse", "Brittle stars", "Centipede"]})
    >>> pa.Table.from_pandas(df)
    pyarrow.Table
    year: int64
    n_legs: int64
    animals: string
    ----
    year: [[2020,2022,2019,2021]]
    n_legs: [[2,4,5,100]]
    animals: [["Flamingo","Horse","Brittle stars","Centipede"]]
    Construct a Table from a dictionary of arrays:
    >>> pydict = {'n_legs': n_legs, 'animals': animals}
    >>> pa.Table.from_pydict(pydict)
    pyarrow.Table
    n_legs: int64
    animals: string
    ----
    n_legs: [[2,4,5,100]]
    animals: [["Flamingo","Horse","Brittle stars","Centipede"]]
    >>> pa.Table.from_pydict(pydict).schema
    n_legs: int64
    animals: string
    Construct a Table from a dictionary of arrays with metadata:
    >>> my_metadata={"n_legs": "Number of legs per animal"}
    >>> pa.Table.from_pydict(pydict, metadata=my_metadata).schema
    n_legs: int64
    animals: string
    -- schema metadata --
    n_legs: 'Number of legs per animal'
    Construct a Table from a list of rows:
    >>> pylist = [{'n_legs': 2, 'animals': 'Flamingo'}, {'year': 2021, 'animals': 'Centipede'}]
    >>> pa.Table.from_pylist(pylist)
    pyarrow.Table
    n_legs: int64
    animals: string
    ----
    n_legs: [[2,null]]
    animals: [["Flamingo","Centipede"]]
    Construct a Table from a list of rows with pyarrow schema:
    >>> my_schema = pa.schema([
    ...     pa.field('year', pa.int64()),
    ...     pa.field('n_legs', pa.int64()),
    ...     pa.field('animals', pa.string())],
    ...     metadata={"year": "Year of entry"})
    >>> pa.Table.from_pylist(pylist, schema=my_schema).schema
    year: int64
    n_legs: int64
    animals: string
    -- schema metadata --
    year: 'Year of entry'
    Construct a Table with :func:`pyarrow.table`:
    >>> pa.table([n_legs, animals], names=names)
    pyarrow.Table
    n_legs: int64
    animals: string
    ----
    n_legs: [[2,4,5,100]]
    animals: [["Flamingo","Horse","Brittle stars","Centipede"]]
    """

    def filter(self, mask, null_selection_behavior="drop") -> Table:
        """
        Select rows from the table.
        See :func:`pyarrow.compute.filter` for full usage.
        Parameters
        ----------
        mask : Array or array-like
            The boolean mask to filter the table with.
        null_selection_behavior
            How nulls in the mask should be handled.
        Returns
        -------
        filtered : Table
            A table of the same schema, with only the rows selected
            by the boolean mask.
        Examples
        --------
        >>> import pyarrow as pa
        >>> import pandas as pd
        >>> df = pd.DataFrame({'year': [2020, 2022, 2019, 2021],
        ...                    'n_legs': [2, 4, 5, 100],
        ...                    'animals': ["Flamingo", "Horse", "Brittle stars", "Centipede"]})
        >>> table = pa.Table.from_pandas(df)
        Define a mask and select rows:
        >>> mask=[True, True, False, None]
        >>> table.filter(mask)
        pyarrow.Table
        year: int64
        n_legs: int64
        animals: string
        ----
        year: [[2020,2022]]
        n_legs: [[2,4]]
        animals: [["Flamingo","Horse"]]
        >>> table.filter(mask, null_selection_behavior='emit_null')
        pyarrow.Table
        year: int64
        n_legs: int64
        animals: string
        ----
        year: [[2020,2022,null]]
        n_legs: [[2,4,null]]
        animals: [["Flamingo","Horse",null]]
        """
        ...
    def take(self, indices) -> Table:
        """
        Select rows from the table.
        See :func:`pyarrow.compute.take` for full usage.
        Parameters
        ----------
        indices : Array or array-like
            The indices in the table whose rows will be returned.
        Returns
        -------
        taken : Table
            A table with the same schema, containing the taken rows.
        Examples
        --------
        >>> import pyarrow as pa
        >>> import pandas as pd
        >>> df = pd.DataFrame({'year': [2020, 2022, 2019, 2021],
        ...                    'n_legs': [2, 4, 5, 100],
        ...                    'animals': ["Flamingo", "Horse", "Brittle stars", "Centipede"]})
        >>> table = pa.Table.from_pandas(df)
        >>> table.take([1,3])
        pyarrow.Table
        year: int64
        n_legs: int64
        animals: string
        ----
        year: [[2022,2021]]
        n_legs: [[4,100]]
        animals: [["Horse","Centipede"]]
        """
        ...
    def replace_schema_metadata(self, metadata: dict[AnyStr, AnyStr] | None = None) -> Table:
        """
        Create shallow copy of table by replacing schema
        key-value metadata with the indicated new metadata (which may be None),
        which deletes any existing metadata.

        Args:
            metadata : dict, default None

        Returns:
            table: Table

        Examples
        --------
        >>> import pyarrow as pa
        >>> import pandas as pd
        >>> df = pd.DataFrame({'year': [2020, 2022, 2019, 2021],
        ...                    'n_legs': [2, 4, 5, 100],
        ...                    'animals': ["Flamingo", "Horse", "Brittle stars", "Centipede"]})
        >>> table = pa.Table.from_pandas(df)
        Constructing a Table with pyarrow schema and metadata:
        >>> my_schema = pa.schema([
        ...     pa.field('n_legs', pa.int64()),
        ...     pa.field('animals', pa.string())],
        ...     metadata={"n_legs": "Number of legs per animal"})
        >>> table= pa.table(df, my_schema)
        >>> table.schema
        n_legs: int64
        animals: string
        -- schema metadata --
        n_legs: 'Number of legs per animal'
        pandas: ...
        Create a shallow copy of a Table with deleted schema metadata:
        >>> table.replace_schema_metadata().schema
        n_legs: int64
        animals: string
        Create a shallow copy of a Table with new schema metadata:
        >>> metadata={"animals": "Which animal"}
        >>> table.replace_schema_metadata(metadata = metadata).schema
        n_legs: int64
        animals: string
        -- schema metadata --
        animals: 'Which animal'
        """
        ...
    @classmethod
    def from_pandas(
        cls,
        df: pd.DataFrame,
        schema: Schema | None = None,
        preserve_index: bool | None = None,
        nthreads: int | None = None,
        columns: list[str] | None = None,
        safe: bool = True,
    ) -> Table:
        """
        Convert pandas.DataFrame to an Arrow Table.
        The column types in the resulting Arrow Table are inferred from the
        dtypes of the pandas.Series in the DataFrame. In the case of non-object
        Series, the NumPy dtype is translated to its Arrow equivalent. In the
        case of `object`, we need to guess the datatype by looking at the
        Python objects in this Series.
        Be aware that Series of the `object` dtype don't carry enough
        information to always lead to a meaningful Arrow type. In the case that
        we cannot infer a type, e.g. because the DataFrame is of length 0 or
        the Series only contains None/nan objects, the type is set to
        null. This behavior can be avoided by constructing an explicit schema
        and passing it to this function.

        Args:
            df : pandas.DataFrame
            schema : pyarrow.Schema, optional
                The expected schema of the Arrow Table. This can be used to
                indicate the type of columns if we cannot infer it automatically.
                If passed, the output will have exactly this schema. Columns
                specified in the schema that are not found in the DataFrame columns
                or its index will raise an error. Additional columns or index
                levels in the DataFrame which are not specified in the schema will
                be ignored.
            preserve_index : bool, optional
                Whether to store the index as an additional column in the resulting
                ``Table``. The default of None will store the index as a column,
                except for RangeIndex which is stored as metadata only. Use
                ``preserve_index=True`` to force it to be stored as a column.
            nthreads : int, default None
                If greater than 1, convert columns to Arrow in parallel using
                indicated number of threads. By default, this follows
                :func:`pyarrow.cpu_count` (may use up to system CPU count threads).
            columns : list, optional
                List of column to be converted. If None, use all columns.
            safe : bool, default True
                Check for overflows or other unsafe conversions.

        Returns:
            table: Table

        Examples
        --------
        >>> import pyarrow as pa
        >>> import pandas as pd
        >>> df = pd.DataFrame({'n_legs': [2, 4, 5, 100],
        ...                    'animals': ["Flamingo", "Horse", "Brittle stars", "Centipede"]})
        >>> pa.Table.from_pandas(df)
        pyarrow.Table
        n_legs: int64
        animals: string
        ----
        n_legs: [[2,4,5,100]]
        animals: [["Flamingo","Horse","Brittle stars","Centipede"]]
        """
        ...
    @staticmethod
    def from_pydict(
        mapping, schema: Schema | None = None, metadata: dict[AnyStr, AnyStr] | None = None
    ) -> Table:
        """Construct a Table from Arrow arrays or columns.

        Args:
            mapping : dict or Mapping
                A mapping of strings to Arrays or Python lists.
            schema : Schema, default None
                If not passed, will be inferred from the Mapping values.
            metadata : dict or Mapping, default None
                Optional metadata for the schema (if inferred).

        Returns:
            table: Table

        Examples
        --------
        >>> import pyarrow as pa
        >>> n_legs = pa.array([2, 4, 5, 100])
        >>> animals = pa.array(["Flamingo", "Horse", "Brittle stars", "Centipede"])
        >>> pydict = {'n_legs': n_legs, 'animals': animals}
        Construct a Table from a dictionary of arrays:
        >>> pa.Table.from_pydict(pydict)
        pyarrow.Table
        n_legs: int64
        animals: string
        ----
        n_legs: [[2,4,5,100]]
        animals: [["Flamingo","Horse","Brittle stars","Centipede"]]
        >>> pa.Table.from_pydict(pydict).schema
        n_legs: int64
        animals: string
        Construct a Table from a dictionary of arrays with metadata:
        >>> my_metadata={"n_legs": "Number of legs per animal"}
        >>> pa.Table.from_pydict(pydict, metadata=my_metadata).schema
        n_legs: int64
        animals: string
        -- schema metadata --
        n_legs: 'Number of legs per animal'
        """
        ...
    @staticmethod
    def from_batches(batches, schema: Schema | None = None) -> Table:
        """Construct a Table from a sequence or iterator of Arrow RecordBatches.

        Args:
            batches : sequence or iterator of RecordBatch
                Sequence of RecordBatch to be converted, all schemas must be equal.
            schema : Schema, default None
                If not passed, will be inferred from the first RecordBatch.

        Returns:
            table: Table

        Examples
        --------
        >>> import pyarrow as pa
        >>> n_legs = pa.array([2, 4, 5, 100])
        >>> animals = pa.array(["Flamingo", "Horse", "Brittle stars", "Centipede"])
        >>> batch = pa.record_batch([n_legs, animals], names=names)
        >>> batch.to_pandas()
           n_legs        animals
        0       2       Flamingo
        1       4          Horse
        2       5  Brittle stars
        3     100      Centipede
        Construct a Table from a RecordBatch:
        >>> pa.Table.from_batches([batch])
        pyarrow.Table
        n_legs: int64
        animals: string
        ----
        n_legs: [[2,4,5,100]]
        animals: [["Flamingo","Horse","Brittle stars","Centipede"]]
        Construct a Table from a sequence of RecordBatches:
        >>> pa.Table.from_batches([batch, batch])
        pyarrow.Table
        n_legs: int64
        animals: string
        ----
        n_legs: [[2,4,5,100],[2,4,5,100]]
        animals: [["Flamingo","Horse","Brittle stars","Centipede"],["Flamingo","Horse","Brittle stars","Centipede"]]
        """
    @property
    def schema(self) -> Schema:
        """Schema of the table and its columns.

        Examples:
        --------
        >>> import pyarrow as pa
        >>> import pandas as pd
        >>> df = pd.DataFrame({'n_legs': [2, 4, 5, 100],
        ...                    'animals': ["Flamingo", "Horse", "Brittle stars", "Centipede"]})
        >>> table = pa.Table.from_pandas(df)
        >>> table.schema
        n_legs: int64
        animals: string
        -- schema metadata --
        pandas: '{"index_columns": [{"kind": "range", "name": null, "start": 0, "' ...
        """
        ...

def field(
    name: AnyStr,
    type: DataType,
    nullable: bool = True,
    metadata: dict[AnyStr, AnyStr] | None = None,
) -> Field:
    """Create a pyarrow.Field instance.

    Args:
        name : Name of the field.
        type : Arrow datatype of the field.
        nullable : Whether the field's values are nullable.
        metadata : dict, default None
            Optional field metadata, the keys and values must be coercible to
            bytes.

    Returns:
        field : pyarrow.Field
    """
    ...
