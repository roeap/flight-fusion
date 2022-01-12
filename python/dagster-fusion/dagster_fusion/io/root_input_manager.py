from dagster import root_input_manager

from

@root_input_manager(input_config_schema={"path": str})
def csv_loader(context):
    return read_csv(context.config["path"])
