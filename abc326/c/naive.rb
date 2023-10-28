def geti
  gets.chomp.to_i
end

def getis
  gets.chomp.split(" ").map(&:to_i)
end

N, M = getis
ps = getis.sort

a = Array.new(100) { 0 }
ps.each do |i|
  a[i] += 1
end
ans = 0

0.upto(100) do |x|
  y = x + M
  tmp = 0
  (x...y).each do |i|
    break unless a[i]
    tmp += a[i]
  end
  ans = tmp if ans < tmp
end

puts ans
