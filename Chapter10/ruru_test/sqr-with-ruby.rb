class Float
  def square_root(e=0.00000001)
    x=0
    while x**2 < (self-e) or x**2 > (self+e)
      x += e
    end
    x 
  end
end

S=9.0
puts S.square_root

