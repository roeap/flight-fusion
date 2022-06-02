class FlightFusionError(Exception):
    pass


class TableSource(FlightFusionError):
    pass


class ConfigError(FlightFusionError):
    pass


class ResourceDoesNotExist(FlightFusionError):
    pass
