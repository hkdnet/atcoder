def geti
  gets.chomp.to_i
end

def getis
  gets.chomp.split(" ").map(&:to_i)
end

n = geti
f = true
while n != 1
  if n % 2 == 0
    n /= 2
    next
  end
  if n % 3 == 0
    n /= 3
    next
  end

  f = false
  break
end

puts(f ? "Yes": "No")
