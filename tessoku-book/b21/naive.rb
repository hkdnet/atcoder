def geti
  gets.chomp.to_i
end

def getis
  gets.chomp.split(" ").map(&:to_i)
end
N = geti
S = gets.chomp
cs = S.chars
ans = 0
(1 << N).times do |s|
  used = cs.each.with_index.filter_map do |c, i|
    if s & (1 << i) > 0
      c
    else
      nil
    end
  end
  s = used.join
  if s == s.reverse
    ans = s.size if ans < s.size
  end
end


puts ans
