def getl = gets.chomp
def geti = getl.to_i
def getis = getl.split(" ").map(&:to_i)
def getis1 = getl.split(" ").map { |e| e.to_i - 1 }

cnt = getl.chars.tally
rev = {}
cnt.each do |c, cnt|
  rev[cnt] ||= []
  rev[cnt] << c
end

f = rev.all? do |_, v|
  l = v.size
  l == 0 || l == 2
end

if f
  puts "Yes"
else
  puts "No"
end
