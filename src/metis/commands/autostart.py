import typer

from metis.autostart.factory import get_autostart

app = typer.Typer()


@app.command()
def enable():
    get_autostart().enable()


@app.command()
def disable():
    get_autostart().disable()


@app.command()
def status():
    enabled = get_autostart().is_enabled()
    print("enabled" if enabled else "disabled")
