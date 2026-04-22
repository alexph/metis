from abc import ABC, abstractmethod


class AutoStart(ABC):
    @abstractmethod
    def enable(self) -> None:
        ...

    @abstractmethod
    def disable(self) -> None:
        ...

    @abstractmethod
    def is_enabled(self) -> bool:
        ...
