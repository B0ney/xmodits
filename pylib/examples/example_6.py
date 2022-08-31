import xmodits
import os

try:
    os.mkdir("pylib/examples/samples/example_6")
except:
    pass

mod = "tests/mods/xm/xo-sat.xm"
folder = "pylib/examples/samples/example_6"

xmodits.dump(
    mod,
    folder,

    index_padding=0,
    index_raw=True,
    index_only=True,
)