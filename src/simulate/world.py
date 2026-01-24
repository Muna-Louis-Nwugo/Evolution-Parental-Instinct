from blip import Blip
import move
import random
import logging

logging.basicConfig(level=logging.INFO)

"""
Simulator world
"""

class World():
    def __init__(
            self, 
            num_starting_blips: int,
            width: int,
            height: int):
        
        self.width = width
        self.height = height

        self.blips: list[Blip] = self.__generate_blips(num_starting_blips)


    def __generate_blips(self, num) -> list[Blip]:
        blips: list[Blip] = []

        for i in range(num):
            x: float = random.randrange(0, self.width)
            y: float = random.randrange(0, self.height)

            blips.append(Blip((x, y)))
        
        return blips
    

    """
    Calls relevant function to move all blips
    """
    def move_blips(self) -> None:
        move.move_blips(self.blips, self.width, self.height)


if __name__ == "__main__":  
    world = World(1, 12, 12)
    world.move_blips()