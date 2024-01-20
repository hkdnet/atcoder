def geti
  gets.chomp.to_i
end

def getis
  gets.chomp.split(" ").map(&:to_i)
end

S = gets.chomp
a = %w[A B C]
cur = 0
cs = S.chars
cs = cs.drop_while { |e| e == "A" }
cs = cs.drop_while { |e| e == "B" }
cs = cs.drop_while { |e| e == "C" }

if cs.size == 0
  puts "Yes"
else
  puts "No"
end
