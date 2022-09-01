import xmodits
import os

try:
    os.mkdir("pylib/examples/samples/example_6")
except:
    pass

# mod = "tests/mods/xm/xo-sat.xm"
mod = "tests/mods/s3m/hip_-_640k_of_space.s3m"

folder = "pylib/examples/samples/example_6"

import glob

try:
    xmodits.dump_multiple(["tests/mods/s3m/hip_-_640k_of_space.s3m"], folder, with_folder=True, index_only=True)
    # xmodits.dump_multiple(glob.glob("tests/mods/it/" +  "*"), folder, with_folder=True)

except Exception as e:
    print(e)

# xmodits.dump(
#     mod,
#     folder,
#     index_only=True,
#     index_raw=True
# )