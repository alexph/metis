import subprocess
from pathlib import Path

PLIST_NAME = "com.metis.plist"


class MacOSAutoStart:
    def _path(self) -> Path:
        return Path.home() / "Library/LaunchAgents" / PLIST_NAME

    def enable(self) -> None:
        path = self._path()
        path.parent.mkdir(parents=True, exist_ok=True)

        binary = self._binary()

        path.write_text(f"""<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN">
<plist version="1.0">
<dict>
  <key>Label</key>
  <string>com.metis</string>
  <key>ProgramArguments</key>
  <array>
    <string>{binary}</string>
    <string>run</string>
  </array>
  <key>RunAtLoad</key>
  <true/>
</dict>
</plist>
""")

        subprocess.run(["launchctl", "load", str(path)])

    def disable(self) -> None:
        subprocess.run(["launchctl", "unload", str(self._path())], check=False)

    def is_enabled(self) -> bool:
        return self._path().exists()

    def _binary(self) -> str:
        import shutil

        return shutil.which("metis") or "metis"
