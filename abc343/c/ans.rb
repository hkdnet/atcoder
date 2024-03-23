def geti
  gets.chomp.to_i
end

def getis
  gets.chomp.split(" ").map(&:to_i)
end

N = geti

ans = nil
(1..).each do |x|
  k = x * x * x
  if k > N
    break
  end
  next unless k.to_s == k.to_s.reverse
  ans ||= k
  ans = k if ans < k
end

puts ans
