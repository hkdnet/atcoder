def geti
  gets.chomp.to_i
end

def getis
  gets.chomp.split(" ").map(&:to_i)
end

N, M, L = getis
A = getis
B = getis
d = Hash.new{|h,k| h[k] = {} }
L.times do
  x, y = getis
  d[x-1][y-1] = true
  d[y-1][x-1] = true
end
sorted_a = A.map.with_index.sort_by{|e| -e[0]}
sorted_b = B.map.with_index.sort_by{|e| -e[0]}

ans = 0
sorted_a.each do |a, ai|
  sorted_b.each do |b, bi|
    break if a + b < ans
    next if d[ai][bi]

    ans = a + b
    break
  end
end

puts ans
