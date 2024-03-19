from commands import Commands, command_implementations

class CommandNotRecognisedException(Exception):
    def __init__(self, command):
        super().__init__(f"{command} not recognised!")

def read_input(url):
    with open(url) as fh:
        data = fh.read().split('\n')
    data = list(map(lambda x: x.strip(), data))
    return list(filter(lambda x: len(x) > 0 and not x.startswith('#'), data))

def execute_command(command, *args):
    match command:
        case 'P': return command_implementations[Commands.SELECT_PEN].call(*args)

        case 'U': return command_implementations[Commands.PEN_UP].call(*args)
        case 'D': return command_implementations[Commands.PEN_DOWN].call(*args)

        case 'W': return command_implementations[Commands.MOVE_WEST].call(*args)
        case 'E': return command_implementations[Commands.MOVE_EAST].call(*args)
        case 'S': return command_implementations[Commands.MOVE_SOUTH].call(*args)
        case 'N': return command_implementations[Commands.MOVE_NORTH].call(*args)

        case _: raise CommandNotRecognisedException(command)

def parse(program):
    output = ""
    for line in program:
        tokens = line.split(' ')
        command = tokens[0]
        args = tokens[1:]

        output += execute_command(command, *args) + "\n"
    return output[:-1]

if __name__ == "__main__":
    url = 'sample.logo'
    output = parse(read_input(url))
    print(output)
