from blip import Blip

"""
This module contains all logic for moving blips
"""

"""
update the position of a given blip

Properties:
Blip - a blip
offset - how much the blip moves
"""
def update_pos(blip: Blip, offset: tuple[float, float]) -> None:
    blip.update_pos(offset)

"""
Moves a group of blips
"""
@staticmethod
def move_blips(blips: list[Blip]) -> None:
    for blip in blips:
        update_pos(blip, (0, 1))