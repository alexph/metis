import os
from pathlib import Path


class WindowsAutoStart:
    def _path(self) -> Path:
        startup = os.environ["APPDATA"]
        return Path(startup) / "Microsoft/Windows/Start Menu/Programs/Startup/metis.bat"

    def enable(self) -> None:
        path = self._path()
        path.parent.mkdir(parents=True, exist_ok=True)

        path.write_text(f'@echo off\n"{self._binary()}" run\n')

    def disable(self) -> None:
        try:
            self._path().unlink()
        except FileNotFoundError:
            pass

    def is_enabled(self) -> bool:
        return self._path().exists()

    def _binary(self) -> str:
        import shutil

        return shutil.which("metis") or "metis"
