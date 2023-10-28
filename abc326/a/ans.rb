def geti
  gets.chomp.to_i
end

def getis
  gets.chomp.split(" ").map(&:to_i)
end

X, Y = getis
if X > Y
  if X - Y > 3
    puts "No"
  else
    puts "Yes"
  end
else
  if Y - X > 2
    puts "No"
  else
    puts "Yes"
  end
end
