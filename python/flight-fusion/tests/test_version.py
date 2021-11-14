def test_version():
    from flight_fusion import __version__

    assert __version__ > "0.0.0"
