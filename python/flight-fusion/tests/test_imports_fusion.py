import flight_fusion
from flight_fusion._internal import FusionClient


def test_import_flight_fusion():
    assert flight_fusion.__name__ == "flight_fusion"


def test_flight_fusion_python_version():
    assert flight_fusion.__version__ > "0.0.0"


def test_class_module_is_flight_fusion():
    for klass in [
        FusionClient,
    ]:
        assert klass.__module__ == "flight_fusion"
