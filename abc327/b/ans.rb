def geti
  gets.chomp.to_i
end

def getis
  gets.chomp.split(" ").map(&:to_i)
end

B = geti

a = 1
ans = -1
loop do
  b = a **a
  if b == B
    ans = a
  end
  break if b >= B
  a+=1
end

puts ans
