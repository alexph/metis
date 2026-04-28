import tomllib
from pydantic import BaseModel

from .paths import ensure_config_path


class MetisUserConfig(BaseModel): ...


class MetisConfig(BaseModel):
    user: MetisUserConfig


def create_config() -> None:
    config_path = ensure_config_path()
    with open(config_path / "config.toml", "wb") as f:
        f.write(b"")
        f.flush()


def load_config() -> MetisConfig:
    config_path = ensure_config_path()
    config_file = config_path / "config.toml"

    if not config_file.exists():
        create_config()

    with open(config_file, "rb") as f:
        user_config = MetisUserConfig(**tomllib.load(f))

    return MetisConfig(user=user_config)


settings = load_config()
