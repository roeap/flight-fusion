from datetime import datetime

from dagster import (
    build_schedule_from_partitioned_job,
    define_asset_job,
    monthly_partitioned_config,
)

from model_training.assets import DATA_PARTITION


@monthly_partitioned_config(start_date=datetime(2020, 1, 1))
def taxi_load_config(start: datetime, _end: datetime):
    return {"ops": {"raw": {"config": {"date": start.strftime("%Y-%m-%d")}}}}


# data loading job
job_data_prep_unresolved = define_asset_job(
    "data_refinement", selection="*taxi/data/refined", partitions_def=DATA_PARTITION
)

schedule_data_prep = build_schedule_from_partitioned_job(
    job_data_prep_unresolved, name="data_load", description="Load data from public repository"
)
