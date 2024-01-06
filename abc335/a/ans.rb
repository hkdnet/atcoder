def geti
  gets.chomp.to_i
end

def getis
  gets.chomp.split(" ").map(&:to_i)
end

s = gets.chomp
s[-1] = "4"
puts s
