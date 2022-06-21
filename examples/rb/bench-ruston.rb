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
  x.report("json    obj") { N.times { JSON.parse(SIMPLE) } }

  ARRAY = %q([1, 2, 3, 4, 5, 6, 7, 8, 9, 10])
  
  x.report("ruston  ary") { N.times { Ruston.new.parse(ARRAY) } }
  x.report("ruston2 ary") { N.times { r.parse(ARRAY) } }
  x.report("json    ary") { N.times { JSON.parse(ARRAY) } }
  
  TRUE = %q(true)
  
  x.report("ruston  true") { N.times { Ruston.new.parse(TRUE) } }
  x.report("ruston2 true") { N.times { r.parse(TRUE) } }
  x.report("json    true") { N.times { JSON.parse(TRUE) } }

  NUM = %q(123)
  
  x.report("ruston  num") { N.times { Ruston.new.parse(NUM) } }
  x.report("ruston2 num") { N.times { r.parse(NUM) } }
  x.report("json    num") { N.times { JSON.parse(NUM) } }

  STR = %q("true")
  
  x.report("ruston  str") { N.times { Ruston.new.parse(STR) } }
  x.report("ruston2 str") { N.times { r.parse(STR) } }
  x.report("json    str") { N.times { JSON.parse(STR) } }
  
  BIG = File.read("examples/rb/big.json")

  x.report("ruston  big") { M.times { Ruston.new.parse(BIG) } }
  x.report("ruston2 big") { M.times { r.parse(BIG) } }
  x.report("json    big") { M.times { JSON.parse(BIG) } }
end
