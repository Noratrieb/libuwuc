# curl -L https://filippo.io/linux-syscall-table/TABELLA_64.json > table.json
import json
with open ("table.json") as f:
    data = f.read()
data=json.loads(data)
for entry in data:
    value = data[entry]
    print(f"pub const SYS_{value[1].upper()}: i64 = {value[0]};")