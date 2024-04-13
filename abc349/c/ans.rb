def getl = gets.chomp
def geti = getl.to_i
def getis = getl.split(" ").map(&:to_i)
def getis1 = getl.split(" ").map { |e| e.to_i - 1 }

S = getl.upcase.chars
T = getl.upcase.chars

idx = 0
S.each do |c|
  if c == T[idx]
    idx += 1
    break if idx == 3
  end
end

if idx == 3 || (idx == 2 && T[2] == 'X')
  puts "Yes"
else
  puts "No"
end
