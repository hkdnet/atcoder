def geti
  gets.chomp.to_i
end

def getis
  gets.chomp.split(" ").map(&:to_i)
end

f = gets.chomp.split("").each_slice(2).map(&:last).all? { |e| e == ?0 }
puts(f ? "Yes" : "No")
