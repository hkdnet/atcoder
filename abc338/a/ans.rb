def geti
  gets.chomp.to_i
end

def getis
  gets.chomp.split(" ").map(&:to_i)
end

S = gets.chomp.chars
if S[0] <= 'Z' && S[1..].all? { |c| c > 'Z' }
  puts "Yes"
else
  puts "No"
end
