import pathlib

from platformdirs import user_cache_path, user_config_path, user_data_path


def ensure_config_path() -> pathlib.Path:
    return user_config_path("metis", "metis", ensure_exists=True)


def ensure_data_path() -> pathlib.Path:
    return user_data_path("metis", "metis", ensure_exists=True)


def ensure_cache_path() -> pathlib.Path:
    return user_cache_path("metis", "metis", ensure_exists=True)
