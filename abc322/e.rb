def geti
  gets.chomp.to_i
end

def getis
  gets.chomp.split(" ").map(&:to_i)
end

N, K, P = getis
plans = N.times.map do
  c, parameters = getis
  {cost:, parameters:}
end
