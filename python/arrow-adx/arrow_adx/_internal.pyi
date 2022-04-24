from __future__ import annotations

from typing import Callable

import pyarrow as pa

rust_core_version: Callable[[], str]

class KustoClient:
    def __init__(self, service_url: str) -> None:
        """Create a new instance of a KustoClient"""
        ...
    @classmethod
    def with_aad_application_key_authentication(
        cls, connection_string: str, aad_app_id: str, app_key: str, authority_id: str
    ) -> KustoClient:
        """Creates a KustoConnection string builder that will authenticate with AAD application and key.

        Args:
            connection_string: Kusto connection string should be of the format: https://<clusterName>.kusto.windows.net
            aad_app_id: AAD application ID.
            app_key: Corresponding key of the AAD application.
            authority_id: Authority id (aka Tenant id) must be provided

        Returns:
            KustoClient: The client instance
        """
        ...
    def execute(self, database: str, query: str) -> list[pa.RecordBatch]:
        """_summary_

        Args:
            database (str): Name of the database in scope that is the target of the query
            query (str): Text of the query to execute

        Returns:
            list[pa.RecordBatch]: Primary query results as list of RecordBatches.
        """
        ...
