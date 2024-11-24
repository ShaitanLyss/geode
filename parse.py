from strictyaml import dirty_load
from path import Path
from pprint import pprint


parsed = dirty_load(Path("attempt-1.yaml").read_text(), allow_flow_style=True )
pprint(parsed.data)

