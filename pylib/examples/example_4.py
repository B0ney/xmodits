import xmodits
import os

try:
    os.mkdir("pylib/examples/samples/example_4")
except:
    pass

mod = "tests/mods/it/before_the_explozion.it"
folder = "pylib/examples/samples/example_4"

xmodits.dump(
    mod,
    folder,
    
    index_padding=0,
)

