from typing import Self
import random

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

Methods:
Getters:
- get_pos() -> tuple[float, float]

Modifiers:
- update_pos(offset: tuple[float, float]) -> None 
    ==> changes this Blips Position

"""

class Blip():
    def __init__(self,  
                 position: tuple[float, float],
                 father: Self | None = None,
                 mother: Self| None = None,
                 velocity: tuple[float, float] = (0, 0),
                 acceleration: tuple[float, float] = (0, 0)):
        self.position = position
        self.age = 0
        self.father = father
        self.mother = mother
        self.sex = random.choice([True, False])
        self.velocity = velocity
        self.acceleration = acceleration
    
    def __repr__(self) -> str:
        return f"Blip(pos={self.position}, velocity={self.velocity}, acceleration={self.acceleration} sex={self.sex}, age={self.age})"

    """
    gets this blip's position
    """
    def get_pos(self) -> tuple[float, float]:
        return self.position

    """
    Updates this blip's position
    """
    def update_pos(self, offset: tuple[float, float]) -> None:
        x = self.position[0]
        y = self.position[1]

        self.position = (x + offset[0], y + offset[1])