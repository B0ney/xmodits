import xmodits
import os

try:
    os.mkdir("pylib/examples/samples/example_3")
except:
    pass

mod = "tests/mods/it/before_the_explozion.it"
folder = "pylib/examples/samples/example_3"

xmodits.dump(
    mod,
    folder,
    
    index_only=True,
)

