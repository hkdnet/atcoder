def geti
  gets.chomp.to_i
end

def getis
  gets.chomp.split(" ").map(&:to_i)
end

N = geti
# h[a][b], a < b
h = Hash.new do |h, k|
  h[k] = {}
end
(N-1).times do
  a, b = getis
  if a > b
    a, b = b, a
  end
  h[a][b] = 0
end
C = getis

