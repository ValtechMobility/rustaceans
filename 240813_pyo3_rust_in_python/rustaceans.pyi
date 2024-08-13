# Type Definitions
from .rustaceans import *

from enum import Enum

class Role(Enum):
    Admin = 1
    User = 2
    Guest = 3

class Location(Enum):
    Munich = 1
    Frankfurt = 2

class Employee():
    def __init__(self, name: str, role: Role, location: Location) -> None: ...