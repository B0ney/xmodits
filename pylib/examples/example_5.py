import xmodits
import os

try:
    os.mkdir("pylib/examples/samples/example_5")
except:
    pass

mod = "tests/mods/xm/xo-sat.xm"
folder = "pylib/examples/samples/example_5"

xmodits.dump(
    mod,
    folder,
    
    index_raw=True,
)

