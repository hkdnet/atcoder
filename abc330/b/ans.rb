def geti
  gets.chomp.to_i
end

def getis
  gets.chomp.split(" ").map(&:to_i)
end

N, L, R = getis
A = getis
a = A.map do |a|
  case
  when a <= L
    L
  when a >= R
    R
  else
    a
  end
end

puts(a.join(" "))
