def geti
  gets.chomp.to_i
end

def getis
  gets.chomp.split(" ").map(&:to_i)
end

N = geti
h = Hash.new {|h,k| h[k] = 0}
b = N.times.map do
  w, x = getis
  h[x] += w
end

ans = 0
(0..24).each do |i|
  tmp = 0
  9.times do |d|
    t = (i + d) % 24
    tmp += h[t]
  end
  ans = tmp if ans < tmp
end

puts ans
