require 'fiddle'

library = Fiddle::dlopen('target/release/libruru_test.so')
Fiddle::Function.new(library['initialize_sum_floats'], [], Fiddle::TYPE_VOIDP).call

5000000.times {
  a=1.2+1.3
}
