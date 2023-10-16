"""
===================
Simple frame parser
===================

:Authors: - Florian Dupeyron <florian.dupeyron@mugcat.fr>
:Date: March 2023
"""

from .framing import Code, MsgFrame

from .        import common, gpio

__CODE_DICT = {
    Code.Ping:        common.RequestPing,
    Code.ItfType:     common.RequestItfType,
    Code.Version:     common.RequestVersion,
    Code.IdGet:       common.RequestIdGet,

    Code.GpioDirSet:  gpio.GpioRequestDirSet,
    Code.GpioDirGet:  gpio.GpioRequestDirGet,
    Code.GpioRead:    gpio.GpioRequestRead,
    Code.GpioWrite:   gpio.GpioRequestWrite,

    Code.GpioValue:   gpio.GpioResponseValue,
    Code.GpioDir:     gpio.GpioResponseDir,

    Code.VersionResp: common.ResponseVersion,
    Code.ItfTypeResp: common.ResponseItfType,

    Code.Good:        common.StatusGood,
    Code.ErrGeneric:  common.StatusErrGeneric,
    # Code.ErrCRC: TODO
    # Code.ErrUnknownCode: TODO
    # Code.ErrInvalidArgs: TODO
    # Code.ErrBusy: TODO
}

def from_frame(frame: MsgFrame):
    cls = __CODE_DICT.get(frame.code, None)

    if cls is None:
        raise KeyError(f"Unsupported code: {frame.code}")

    return cls.from_frame(frame)
