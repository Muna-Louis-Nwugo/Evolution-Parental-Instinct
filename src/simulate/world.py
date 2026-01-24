from blip import Blip
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
            width,
            height):
        
        self.width = width
        self.height = height

        self.blips: list[Blip] = self.__generate_blips(num_starting_blips)
        logging.info(f"blips: {self.blips}")


    def __generate_blips(self, num) -> list[Blip]:
        blips: list[Blip] = []

        for i in range(num):
            x: float = random.randrange(0, self.width)
            y: float = random.randrange(0, self.height)

            blips.append(Blip((x, y)))
        
        return blips
    
World(12, 12, 12)