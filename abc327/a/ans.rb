def geti
  gets.chomp.to_i
end

def getis
  gets.chomp.split(" ").map(&:to_i)
end

N = geti
S= gets.chomp
if S["ab"]  || S["ba"]
  puts "Yes"
else
  puts "No"
end
