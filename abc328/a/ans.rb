def geti
  gets.chomp.to_i
end

def getis
  gets.chomp.split(" ").map(&:to_i)
end

N, X = getis
S = getis

puts(S.select{|e| e <= X}.sum)
