"""
This module contains all logic for moving blips
"""

from blip import Blip
import random
import logging

logging.basicConfig(level=logging.INFO)

def update_accel(blip: Blip):
    target: tuple[float, float] | None = blip.get_target()
    blip_pos: tuple[float, float] = blip.get_pos()

    if not target:
        return

    desired_accel: tuple[float, float] = (target[0] - blip_pos[0], target[1] - blip_pos[1])

    blip.update_accel(desired_accel)



def move_blips(blips: list[Blip], map_width: int, map_height: int) -> None:
    """
    Moves a group of blips. This is done by applying an acceleration value to each of the blips

    blips: list[Blip] ==> a list of blips to be moved
    map_width: int ==> width of map (boundary detection)
    map_heigh: int ==> height of map (boundary detection)
    """
    
    for blip in blips:
        set_target(blip, map_width, map_height)
        update_accel(blip)



def set_target(blip: Blip, map_width: int, map_height: int) -> None:
    """
    sets the target of a given blip

    blip: Blip ==> the blip to be affected
    map_width: int ==> width of map (boundary detection)
    map_heigh: int ==> height of map (boundary detection)
    """
    if blip.get_target is not None:
        x = random.randrange(0, map_width)
        y = random.randrange(0, map_height)

        blip.set_target((x, y))

    else:
        pass
