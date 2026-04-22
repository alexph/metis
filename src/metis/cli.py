import typer

from metis.app.run import main
from metis.commands import autostart

app = typer.Typer()
app.add_typer(autostart.app, name="autostart")


@app.command()
def start():
    main()


if __name__ == "__main__":
    app()
