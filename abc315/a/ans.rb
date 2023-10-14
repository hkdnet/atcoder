def geti
  gets.chomp.to_i
end

def getis
  gets.chomp.split(" ").map(&:to_i)
end

s = gets.split("").reject{|e| %w[a i u e o].include?(e) }.join("")
puts s
