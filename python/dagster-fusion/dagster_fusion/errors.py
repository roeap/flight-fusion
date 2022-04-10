class DagsterFusionError(Exception):
    """Generic error raised for exception within dagster-fusion"""

    pass


class MissingConfiguration(DagsterFusionError):
    """An expected configuration key is not available"""

    pass
