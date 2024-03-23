def geti
  gets.chomp.to_i
end

def getis
  gets.chomp.split(" ").map(&:to_i)
end

S = gets.chomp
if S[0] == "<" && S[-1] == ">" && S[1, S.size - 2].chars.all? {|c| c == "=" }
  puts "Yes"
else
  puts "No"
end
