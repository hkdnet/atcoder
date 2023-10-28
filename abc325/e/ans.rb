def geti
  gets.chomp.to_i
end

def getis
  gets.chomp.split(" ").map(&:to_i)
end


N, A, B, C = getis
m = N.times.map do
  getis
end
