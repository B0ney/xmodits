import xmodits
import os

try:
    os.mkdir("pylib/examples/samples/example_6")
except:
    pass

# mod = "tests/mods/xm/xo-sat.xm"
mod = "tests/mods/s3m/space_odyssey_v1_2.s3m"

folder = "pylib/examples/samples/example_6"

xmodits.dump(
    mod,
    folder,
    index_only=True,
)