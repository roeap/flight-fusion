import flight_fusion_server
from flight_fusion_server._internal import FusionServer


def test_import_flight_fusion_server():
    assert flight_fusion_server.__name__ == "flight_fusion_server"


def test_flight_fusion_python_version():
    assert flight_fusion_server.__version__ > "0.0.0"


def test_class_module_is_flight_fusion():
    for klass in [
        FusionServer,
    ]:
        assert klass.__module__ == "flight_fusion_server"
