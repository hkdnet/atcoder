def geti
  gets.chomp.to_i
end

def getis
  gets.chomp.split(" ").map(&:to_i)
end

S = gets.chomp
h = S.chars.tally
cnt = 0
ans = nil
h.keys.sort.each do |c|
  v = h[c]
  if cnt < v
    ans = c
    cnt = v
  end
end

puts ans
