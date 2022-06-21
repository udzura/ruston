require 'benchmark'
require 'ruston'
require 'json'

N = 100000
M = 10

Benchmark.bmbm do |x|
  SIMPLE = %q({"Hello": "world", "lireral": true, "numeric": 1})
  r = Ruston.new
  
  x.report("ruston  obj") { N.times { Ruston.new.parse(SIMPLE) } }
  x.report("ruston2 obj") { N.times { r.parse(SIMPLE) } }
  x.report("json    obj")  { N.times { JSON.parse(SIMPLE) } }

  ARRAY = %q([1, 2, 3, 4, 5, 6, 7, 8, 9, 10])
  
  x.report("ruston  ary") { N.times { Ruston.new.parse(ARRAY) } }
  x.report("ruston2 ary") { N.times { r.parse(ARRAY) } }
  x.report("json    ary")  { N.times { JSON.parse(ARRAY) } }
  
  JUST_LIT = %q(true)
  
  x.report("ruston  true") { N.times { Ruston.new.parse(JUST_LIT) } }
  x.report("ruston2 true") { N.times { r.parse(JUST_LIT) } }
  x.report("json    true")  { N.times { JSON.parse(JUST_LIT) } }

  BIG = File.read("examples/rb/big.json")

  x.report("ruston  big") { M.times { Ruston.new.parse(BIG) } }
  x.report("ruston2 big") { M.times { r.parse(BIG) } }
  x.report("json    big")  { M.times { JSON.parse(BIG) } }
end
