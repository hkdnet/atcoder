def geti
  gets.chomp.to_i
end

def getis
  gets.chomp.split(" ").map(&:to_i)
end

N = geti
A = N.times.map do
  geti
end
h = {}
A.each do |a|
  h[a] ||= 0
  h[a] += 1
end

sorted = h.keys.sort
