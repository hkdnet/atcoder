def geti
  gets.chomp.to_i
end

def getis
  gets.chomp.split(" ").map(&:to_i)
end

Q = geti

a = []
Q.times do
  ty, x = getis
  if ty == 1
    a << x
  else
    puts a[-x]
  end
end
