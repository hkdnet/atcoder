def getl = gets.chomp
def geti = getl.to_i
def getis = getl.split(" ").map(&:to_i)
def getis1 = getl.split(" ").map { |e| e.to_i - 1 }

H = geti
t = 0
d = 0
while t <= H
  t += 2**d
  d+= 1
end

puts d
