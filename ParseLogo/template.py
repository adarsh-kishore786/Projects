class ArgumentException(Exception):
    def __init__(self, command, num_args, num_actual):
        super().__init(f"{command} supports {num_args} arguments but {num_actual} supplied")

class JavaScriptCommand:
    def __init__(self, command, num_args, function):
        self.command = command
        self.num_args = num_args
        self.function = function

    def call(self, *args):
        if len(args) != self.num_args:
            raise ArgumentException(self.command, self.num_args, self.function)

        return self.function(*args)
