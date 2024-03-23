def geti
  gets.chomp.to_i
end

def getis
  gets.chomp.split(" ").map(&:to_i)
end

N = geti
S = gets.chomp.chars
C = getis

ans = Float::INFINITY
(1 << N).times do |s|
  s = s.to_s(2)
  if s.size < N
    s = "0" * (N - s.size) + s
  end
  sum = s.chars.each_cons(2).map do |a, b|
    a == b ? 1 : 0
  end.sum
  next if sum != 1

  # p s
  tmp = 0
  s.chars.each.with_index do |c, i|
    if S[i] != c
      tmp += C[i]
    end
  end
  # p tmp
  if tmp < ans
    ans = tmp
  end
end

puts ans
