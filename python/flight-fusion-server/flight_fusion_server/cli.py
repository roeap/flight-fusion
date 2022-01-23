import typer

from flight_fusion_server import FusionServer

app = typer.Typer()


@app.command()
def start():
    typer.echo("Starting flight fusion server")
    server = FusionServer()
    server.run()
