def geti
  gets.chomp.to_i
end

def getis
  gets.chomp.split(" ").map(&:to_i)
end

N, K = getis
balls = N.times.map do
  getis
end

a = N.times.to_a
ans = -1
a.permutation(K).each do |is|
  arr = []
  balls.each.with_index do |(c,v), i|
    next if is.include?(i)
    arr << [c, v]
  end
  f = true
  arr.each_cons(2) do |a, b|
    if a[0] == b[0]
      f =false
      break
    end
  end

  if f
    sum = arr.sum{|a| a[1]}
    ans = sum if ans < sum
  end
end

puts ans
