def getl = gets.chomp
def geti = getl.to_i
def getis = getl.split(" ").map(&:to_i)
def getis1 = getl.split(" ").map { |e| e.to_i - 1 }

N, M = getis
H = getis
tmp = M
ans = 0
H.each do |h|
  tmp -= h
  break if tmp < 0
  ans += 1
end
puts ans
