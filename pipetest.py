import subprocess
import struct


print()

json = r"{\"hello\": 1}"
l = len(json)
header = struct.pack("=I", l)
body = json.encode("utf-8")


proc = subprocess.Popen(["cargo", "run", "--bin", "rofitab"],
                        stdin=subprocess.PIPE)

proc.communicate(input=header+body)
