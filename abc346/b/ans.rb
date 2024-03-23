def geti
  gets.chomp.to_i
end

def getis
  gets.chomp.split(" ").map(&:to_i)
end

pat = "wbwbwwbwbwbw"

W, B = getis
S = pat * 1000
len = W + B
f = false
(0..).each do |l|
  str = S[l, len]
  break if str.size != len
  if str.count("w") == W && str.count("b") == B
    f = true
    break
  end
end

if f
  puts "Yes"
else
  puts "No"
end
