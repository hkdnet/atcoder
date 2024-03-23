def geti
  gets.chomp.to_i
end

def getis
  gets.chomp.split(" ").map(&:to_i)
end

H, W, M = getis
colors = M.times.map do
 t, a, x = getis
end
h = Hash.new(0)
used = Hash.new do |h,k|
  h[k] = {}
end
colors.reverse_each do |ty, i, x|
  if used[ty][i]
    #puts "skip"
    next
  end

  base = ty == 1 ? W : H
  delta = base - used[ty == 1 ? 2 : 1].size
  h[x] += delta if delta > 0

  used[ty][i] = true
  # p h
end

missed = H * W - h.values.sum
h[0] += missed if missed > 0

h.delete_if { |k, v| v <= 0 }
puts h.keys.size
h.keys.sort.each do |k|
  v = h[k]
  raise if v <= 0
  puts "#{k} #{v}"
end
