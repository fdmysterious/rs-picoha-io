"""
=========================================
HA protocol requests and reponse for GPIO
=========================================

:Authors: - Florian Dupeyron <florian.dupeyron@mugcat.fr>
:Date: March 2023
"""

from dataclasses import dataclass
from enum        import IntEnum

from .framing    import (MsgFrame, Code)


# ┌────────────────────────────────────────┐
# │ Specific types                         │
# └────────────────────────────────────────┘

class GpioDir(IntEnum):
    PullDownInput = 0
    PullUpInput   = 1
    Output        = 2


class GpioValue(IntEnum):
    Low           = 0
    High          = 1


# ┌────────────────────────────────────────┐
# │ GPIO requests                          │
# └────────────────────────────────────────┘

@dataclass
class GpioRequestDirSet:
    pin: int
    dir: GpioDir

    def to_frame(self) -> MsgFrame:
        return MsgFrame(
            code = Code.GpioDirSet,
            data = bytes([self.pin, self.dir.value])
        )

    @classmethod
    def from_frame(cls, frame: MsgFrame) -> "GpioRequestDirSet":
        if frame.code != Code.GpioDirSet:
            raise ValueError(f"Invalid code for {cls}: {frame.code}")
        return cls( pin = frame.data[0], dir = GpioDir(frame.data[1]) )


@dataclass
class GpioRequestDirGet:
    pin: int

    def to_frame(self) -> MsgFrame:
        return MsgFrame(
            code = Code.GpioDirGet,
            data = bytes([self.pin])
        )

    @classmethod
    def from_frame(cls, frame: MsgFrame) -> "GpioRequestDirGet":
        if frame.code != Code.GpioDirGet:
            raise ValueError(f"Invalid code for {cls}: {frame.code}")
        return cls(pin = frame.data[0])


@dataclass
class GpioRequestWrite:
    pin: int
    value: GpioValue

    def to_frame(self) -> MsgFrame:
        return MsgFrame(
            code = Code.GpioWrite,
            data = bytes([self.pin, self.value.value])
        )

    @classmethod
    def from_frame(cls, frame: MsgFrame) -> "GpioRequestWrite":
        if frame.code != Code.GpioWrite:
            raise ValueError(f"Invalid code for {cls}: {frame.code}")
        return cls(pin = frame.data[0], value = GpioValue(frame.data[1]))

@dataclass
class GpioRequestRead:
    pin: int

    def to_frame(self) -> MsgFrame:
        return MsgFrame(
            code = Code.GpioRead,
            data = bytes([self.pin, self.value.value])
        )

    @classmethod
    def from_frame(cls, frame: MsgFrame) -> "GpioRequestRead":
        if frame.code != Code.GpioRead:
            raise ValueError(f"Invalid code for {cls}: {frame.code}")
        return cls(pin = frame.data[0])

# ┌────────────────────────────────────────┐
# │ GPIO responses                         │
# └────────────────────────────────────────┘

@dataclass
class GpioResponseValue:
    pin: int
    value: GpioValue

    def to_frame(self) -> MsgFrame:
        return MsgFrame(
            code = Code.GpioValue,
            data = bytes([self.pin, self.value.value])
        )

    @classmethod
    def from_frame(cls, frame: MsgFrame) -> "GpioResponseValue":
        if frame.code != Code.GpioValue:
            raise ValueError(f"Invalid code for {cls}: {frame.code}")
        return cls(pin = frame.data[0], value = GpioValue(frame.data[1]))


@dataclass
class GpioResponseDir:
    pin: int
    value: GpioDir

    def to_frame(self) -> MsgFrame:
        return MsgFrame(
            code = Code.GpioDir,
            data = bytes([self.pin, self.value.value])
        )

    @classmethod
    def from_frame(cls, frame: MsgFrame) -> "GpioResponseDir":
        if frame.code != Code.GpioValue:
            raise ValueError(f"Invalid code for {cls}: frame.code")

        return cls(pin = frame.data[0], value = GpioDir(frame.data[1]))
