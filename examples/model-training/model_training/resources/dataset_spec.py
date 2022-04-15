from dagster import make_values_resource

dataset_properties = make_values_resource(n_samples=int, test_split=float)
