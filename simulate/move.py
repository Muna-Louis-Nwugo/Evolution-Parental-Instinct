from blip import Blip

"""
This module contains all logic for moving blips
"""

class Move():
    def __init__(self):
        pass


    """
    update the position of a given blip

    Properties:
    Blip - a blip
    offset - how much the blip moves
    """
    def update_pos(self, blip: Blip, offset: tuple[float, float]) -> None:
        blip.update_pos(offset)