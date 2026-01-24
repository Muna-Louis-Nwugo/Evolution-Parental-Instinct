from typing import Self

"""
This module contains the class for creating a blip.

Properties:
- Age: int ==> this blip's age in days from birth
- Father: Blip ==> this blip's father
- Mother: Blip ==> this blip's mother
- Sex: bool ==> this blip's sex. True is male, false is female
- Position: tuple[float, float] ==> this blip's position on the map
- Velocity: tuple[float, float] ==> this blip's velocity
- Acceleration: tuple[float, float] ==> this blip's acceleration
"""

class Blip():
    def __init__(self, 
                 age: int, 
                 father: Self, 
                 mother: Self, 
                 sex: bool,
                 position: tuple[float, float],
                 velocity: tuple[float, float],
                 acceleration: tuple[float, float]):
        self.age = age
        self.father = father
        self.mother = mother
        self.sex = sex
        self.position = position
        self.velocity = velocity
        self.acceleration = acceleration

    """
    Updates this blip's position
    """
    def update_pos(self, offset):
        x = self.position[0]
        y = self.position[1]

        self.position = (x + offset, y + offset)