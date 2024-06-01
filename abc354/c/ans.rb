def getl = gets.chomp
def geti = getl.to_i
def getis = getl.split(" ").map(&:to_i)
def getis1 = getl.split(" ").map { |e| e.to_i - 1 }

N = geti
cards = N.times.map do |i|
  a, c = getis
  { power: a, cost: c, id: i + 1 }
end
sorted = cards.sort_by { |e| [-e[:power], e[:cost]] }

# x
power = sorted[0][:power]
cost = sorted[0][:cost]
discarded = Set.new
sorted.drop(1).each do |card|
  if power > card[:power]
    if cost < card[:cost]
      discarded << card[:id]
    end
  end
  if card[:cost] < cost
     cost = card[:cost]
  end
end

a = 1.upto(N).reject { |i| discarded.include?(i) }

puts a.size
puts a.join(" ")
