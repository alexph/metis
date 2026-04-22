import subprocess
from pathlib import Path

SERVICE_NAME = "metis.service"


class LinuxAutoStart:
    def _service_path(self) -> Path:
        return Path.home() / ".config/systemd/user" / SERVICE_NAME

    def enable(self) -> None:
        path = self._service_path()
        path.parent.mkdir(parents=True, exist_ok=True)

        path.write_text(
            f"""
[Unit]
Description=Metis

[Service]
ExecStart={self._binary()} run
Restart=always

[Install]
WantedBy=default.target
""".strip()
        )

        subprocess.run(["systemctl", "--user", "daemon-reexec"], check=False)
        subprocess.run(["systemctl", "--user", "enable", SERVICE_NAME])
        subprocess.run(["systemctl", "--user", "start", SERVICE_NAME])

    def disable(self) -> None:
        subprocess.run(["systemctl", "--user", "disable", "--now", SERVICE_NAME], check=False)

    def is_enabled(self) -> bool:
        return self._service_path().exists()

    def _binary(self) -> str:
        import shutil

        return shutil.which("metis") or "metis"
