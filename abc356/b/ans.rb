def getl = gets.chomp
def geti = getl.to_i
def getis = getl.split(" ").map(&:to_i)
def getis1 = getl.split(" ").map { |e| e.to_i - 1 }

N, M = getis
A = getis
arr = [0] * M
N.times.map do
  tmp = getis
  arr = arr.zip(tmp).map do |a, e|
    a + e
  end
end
f = A.zip(arr).all? do |t, a|
  t <= a
end

if f
  puts "Yes"
else
  puts "No"
end
