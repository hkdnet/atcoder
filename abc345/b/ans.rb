def geti
  gets.chomp.to_i
end

def getis
  gets.chomp.split(" ").map(&:to_i)
end

X = geti
if X == 0
  puts 0
elsif X > 0
  z = X % 10 == 0
  a = X / 10
  a += 1 unless z
  puts a
else
  z = X % 10 == 0
  a = X / 10
  if z
    puts a
  else
    puts(a+1)
  end

end
