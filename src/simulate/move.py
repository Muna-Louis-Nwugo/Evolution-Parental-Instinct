from blip import Blip
import random
import logging

logging.basicConfig(level=logging.INFO)

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
Moves a group of blips.

blips: list[Blip] ==> a list of blips to be moved
map_width: float ==> width of map (boundary detection)
map_heigh: float ==> height of map (boundary detection)
"""
@staticmethod
def move_blips(blips: list[Blip], map_width: int, map_height: int) -> None:
    
    for blip in blips:
        set_target(blip, map_width, map_height)
        update_pos(blip, (0, 1))

def set_target(blip: Blip, map_width: int, map_height: int) -> None:
    if blip.get_target is not None:
        x = random.randrange(0, map_width)
        y = random.randrange(0, map_height)

        blip.set_target((x, y))

    else:
        pass
