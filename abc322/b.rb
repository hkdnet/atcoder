def geti
  gets.chomp.to_i
end

def getis
  gets.chomp.split(" ").map(&:to_i)
end
_ = gets

S = gets.chomp
T = gets.chomp

v =
  if T.start_with?(S)
    if T.end_with?(S)
      0
    else
      1
    end
  else
    if T.end_with?(S)
      2
    else
      3
    end
  end

puts v
