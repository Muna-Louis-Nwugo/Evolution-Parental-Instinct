from typing import Self

"""
This module contains the class for creating a blip.

States:
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
                 target: tuple[float, float] = (0, 0)):
        self.position = position
        self.velocity = (0,0)
        self.acceleration = (0,0)
        self.max_acceleration = 1
        self.max_velocity = 1
        self.target = target
        
        
    def __repr__(self) -> str:
        return f"Blip(pos={self.position}, velocity={self.velocity}"

    """
    gets this blip's position
    """
    def get_pos(self) -> tuple[float, float]:
        return self.position


    """
    Updates this blip's position
    """
    def update_pos(self, x_offset: float, y_offset: float) -> None:
        x = self.position[0]
        y = self.position[1]

        self.position = (x + x_offset, y + y_offset)
        
 
    def set_target(self, target: tuple[float, float]) -> None:
        self.target = target
