def geti
  gets.chomp.to_i
end

def getis
  gets.chomp.split(" ").map(&:to_i)
end

S = gets.chomp
k, _ = S.chars.tally.each.min_by do |k, v|
  v
end
puts(S.index(k) + 1)
