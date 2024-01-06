def geti
  gets.chomp.to_i
end

def getis
  gets.chomp.split(" ").map(&:to_i)
end

N = geti
A = getis
sorted = A.sort
last_idx = {}
sorted.each.with_index do |a, i|
  last_idx[a] = i
end

acc = [0]
sorted.each.with_index do |e, i|
  acc[i+1] = e + acc[i]
end
b = A.map do |a|
  idx = last_idx[a] + 1
  acc[-1] - acc[idx]
end

puts b.join(" ")
