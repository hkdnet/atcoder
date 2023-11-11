def geti
  gets.chomp.to_i
end

def getis
  gets.chomp.split(" ").map(&:to_i)
end

S = gets.chomp
ss = S.chars

if S.size >= 3
  arr = []
  cs = [nil, nil, S[0], S[1], S[2]]
  nx = 3
  while nx <= S.size + 2 do
    c1, c2, c3 = cs[-3..]
    if c1 == 'A' && c2 == 'B' && c3 == 'C'
      ncs = [arr[-2], arr[-1], cs[0], cs[1], S[nx]]
      arr[-2] = nil
      arr[-1] = nil
      cs = ncs
    else
      c = cs.shift
      arr << c unless c.nil?
      cs << S[nx]
    end
    nx += 1
  end

  s = (arr + cs).join("").gsub("ABC", "")
  puts s
else
  puts S
end

# ABCCAABABABCABCCCBCCABCABCBACCCACA
# CAAB ABABC ABCCCBCCABCABCBACCCACA
# CAAB ABABC CCBCCABCABCBACCCACA
# CAAB ABC CBCCABCABCBACCCACA
