PAT = /A|C|G|T/

s = gets.chomp

max = 0
tmp = 0
s.chars.each do |c|
  if c.match?(PAT)
    tmp += 1
  else
    max = [max, tmp].max
    tmp = 0
  end
end

max = [max, tmp].max

puts max
