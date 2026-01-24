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
- get_target() -> tuple[float, float] | None

Modifiers:
- update_pos(offset: tuple[float, float]) -> None 
    ==> changes this Blips Position
- set_target(target) -> None:
    ==> sets this blip's target

"""

class Blip():
    def __init__(self,  
                 position: tuple[float, float],
                 father: Self | None = None,
                 mother: Self| None = None,
                 velocity: tuple[float, float] = (0, 0),
                 acceleration: tuple[float, float] = (0, 0),
                 target: tuple[float, float] | None = None):
        self.position = position
        self.age = 0
        self.father = father
        self.mother = mother
        self.sex = random.choice([True, False])
        self.velocity = velocity
        self.acceleration = acceleration
        self.target = target
    
    def __repr__(self) -> str:
        return f"Blip(pos={self.position}, velocity={self.velocity}, acceleration={self.acceleration} sex={self.sex}, age={self.age})"

    """
    gets this blip's position
    """
    def get_pos(self) -> tuple[float, float]:
        return self.position
    
    """
    gets this blip's target
    """
    def get_target(self) -> tuple[float, float] | None:
        return self.target

    """
    sets this blip's target
    """
    def set_target(self, target: tuple[float, float]) -> None:
        self.target = target

    """
    Updates this blip's position
    """
    def update_pos(self, offset: tuple[float, float]) -> None:
        x = self.position[0]
        y = self.position[1]

        self.position = (x + offset[0], y + offset[1])