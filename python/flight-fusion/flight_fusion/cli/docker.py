import typer
from loguru import logger

app = typer.Typer(name="docker", help="Configure and run fusion deployment")


@app.command()
def start():
    try:
        from python_on_whales.docker_client import DockerClient
    except ImportError:
        logger.error("Install flight_fusion with extra `docker` to use docker commands")
        typer.Abort("Docker extra not installed")
        raise Exception

    docker = DockerClient(compose_project_name="flight-fusion", compose_files=[])

    logger.info("Checking if docker compose is installed....")
    if not docker.compose.is_installed():
        logger.error("Docker Compose CLI is not installed on the system.")
        typer.Abort()

    docker_info = docker.info()

    logger.info("Checking if docker service is running....")
    if not docker_info.id:
        raise Exception("Docker Service is not up and running.")

    logger.info("starting fusion environment")
