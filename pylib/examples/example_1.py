import xmodits
import os

try:
    os.mkdir("pylib/examples/samples/example_1")
except:
    pass

mod = "tests/mods/it/before_the_explozion.it"
folder = "pylib/examples/samples/example_1"

xmodits.dump(mod, folder)