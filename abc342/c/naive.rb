def geti
  gets.chomp.to_i
end

def getis
  gets.chomp.split(" ").map(&:to_i)
end

N = geti
S = gets.chomp
Q = geti
cs = S.chars

Q.times do
  c, d = gets.chomp.split(" ", 2)
  cs = cs.map do |char|
    if char == c
      d
    else
      char
    end
  end
end

puts cs.join("")
