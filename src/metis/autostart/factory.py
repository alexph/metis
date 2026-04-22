import sys

from .linux import LinuxAutoStart
from .macos import MacOSAutoStart
from .windows import WindowsAutoStart


def get_autostart():
    if sys.platform.startswith("linux"):
        return LinuxAutoStart()
    if sys.platform == "darwin":
        return MacOSAutoStart()
    if sys.platform in ("win32", "cygwin"):
        return WindowsAutoStart()
    raise RuntimeError("Unsupported OS")
