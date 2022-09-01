import xmodits
import os

try:
    os.mkdir("pylib/examples/samples/example_2")
except:
    pass

mod = "tests/mods/it/before_the_explozion.it"
folder = "pylib/examples/samples/example_2"

xmodits.dump(
    mod,
    folder,

    with_folder=True,
)

