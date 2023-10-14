def geti
  gets.chomp.to_i
end

def getis
  gets.chomp.split(" ").map(&:to_i)
end

gets
a = getis
puts(a.all?{|e| e == a[0]} ? "Yes" : "No")
