from . import blip
from . import blip_mover
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
        self.time_elapsed: int = 0

        self.blips: list[blip.Blip] = self.__generate_blips(num_starting_blips)


    def __generate_blips(self, num) -> list[blip.Blip]:
        blips: list[blip.Blip] = []

        for i in range(num):
            x: float = random.randrange(0, self.width)
            y: float = random.randrange(0, self.height)

            blips.append(blip.Blip((x, y)))
        
        return blips
    

    """
    Steps through simulation
    """
    def step(self) -> None:
        print(self.blips)
        blip_mover.move_blips(self.blips, self.width, self.height)
        print(self.blips)


if __name__ == "__main__":  
    world = World(1, 12, 12)

    for i in range(10):
        world.step()
