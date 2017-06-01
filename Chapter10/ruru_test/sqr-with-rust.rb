require 'fiddle'

library = Fiddle::dlopen('target/release/libruru_test.so')
Fiddle::Function.new(library['initialize_sum_floats'], [], Fiddle::TYPE_VOIDP).call

S=9.0
puts S.square_root

