"""
=============================================
Helper class to handle GPIO interface devices
=============================================

:Authors: - Florian Dupeyron <florian.dupeyron@mugcat.fr>
:Date: March 2023
"""

from .          import DeviceWrapper
from ..protocol import gpio, common


class GpioDevice(DeviceWrapper):
    def __init__(self, transport):
        super().__init__(transport)

    def message_callback(self, msg):
        # TODO # Filter GpioFront messages
        return msg

    ############

    def direction_set(self, idx: int, ddir: gpio.GpioDir):
        self.request(gpio.GpioRequestDirSet(idx, ddir), common.StatusGood)


    def direction_get(self, idx: int):
        resp = self.request(gpio.GpioRequestDirGet(idx), gpio.GpioResponseDir)
        if resp.pin != idx:
            raise ValueError(f"Wrong pin response: {resp.pin} != {idx}")

        return resp.value


    def write(self, idx: int, value: gpio.GpioValue):
        self.request(gpio.GpioRequestWrite(idx, value), common.StatusGood)

    def read(self, idx: int):
        resp = self.request(gpio.GpioRequestRead(idx), gpio.GpioResponseValue)

        if resp.pin != idx:
            raise ValueError(f"Wrong pin response: {resp.pin} != {idx}")

        return resp.value
