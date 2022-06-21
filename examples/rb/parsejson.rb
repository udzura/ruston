require 'ruston'
require 'ruston.so'

json = <<JSON
[
{"id": 1,
 "name": "udzura"},
{"id": 2,
 "name": "akubi"}
]
JSON

ret = Ruston.new.parse(json)
p ret
