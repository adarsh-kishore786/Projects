from enum import Enum
from template import JavaScriptCommand

class Commands(Enum):
    SELECT_PEN = 0
    PEN_DOWN   = 1
    MOVE_WEST  = 2
    MOVE_EAST  = 3
    MOVE_NORTH = 4
    MOVE_SOUTH = 5
    PEN_UP     = 6

command_implementations = { c.value: None for c in (Commands) }

def move_function(direction, magnitude):
    return f"Moved {magnitude} units {direction}"

def select_pen(pen_number):
    return f"Selected pen {pen_number}"

def pen_up_down(direction):
    return f"Put pen {direction}"

# define the dictionary of commands with the appropriate functions

# selecting pen
command_implementations[Commands.SELECT_PEN] = JavaScriptCommand('P', 1, select_pen)

# moving pen up and down
command_implementations[Commands.PEN_DOWN]   = JavaScriptCommand('D', 0, lambda: pen_up_down("down"))
command_implementations[Commands.PEN_UP]     = JavaScriptCommand('U', 0, lambda: pen_up_down("up"))

# define movement
command_implementations[Commands.MOVE_WEST]  = JavaScriptCommand('W', 1, lambda mag: move_function("west", mag))
command_implementations[Commands.MOVE_EAST]  = JavaScriptCommand('E', 1, lambda mag: move_function("east", mag))
command_implementations[Commands.MOVE_NORTH] = JavaScriptCommand('N', 1, lambda mag: move_function("north", mag))
command_implementations[Commands.MOVE_SOUTH] = JavaScriptCommand('S', 1, lambda mag: move_function("south", mag))
