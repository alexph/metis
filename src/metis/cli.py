import typer

from metis.app.run import RuntimeOptions, main
from metis.commands import autostart

app = typer.Typer()
app.add_typer(autostart.app, name="autostart")


@app.command()
def start(dev: bool = False):
    runtime_options = RuntimeOptions(dev=dev)
    main(runtime_options)


if __name__ == "__main__":
    app()
