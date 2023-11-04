def geti
  gets.chomp.to_i
end

def getis
  gets.chomp.split(" ").map(&:to_i)
end

N = geti
h = {}
N.times do |i|
  c = geti
  a = getis
  h[i+1] = a
end
X = geti

ps = h.select do |k, v|
  v.include?(X)
end
sorted = ps.sort_by {|k, v| v.size }
if sorted.empty?
  puts "0"
  puts ""
else
  fk, fv = sorted.first
  a = sorted.select {|k, v| v.size == fv.size }
  puts "#{a.size}"
  puts "#{a.map(&:first).join(" ")}"
end
