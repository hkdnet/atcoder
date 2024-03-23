def geti
  gets.chomp.to_i
end

def getis
  gets.chomp.split(" ").map(&:to_i)
end

H, W, M = getis
grid = Hash.new do |hh, k|
  hh[k] = Hash.new(0)
end

M.times.each do
  t, a, x = getis
  a -=1
  if t == 1
    W.times do |k|
      grid[a][k] = x
    end
  else
    H.times do |k|
      grid[k][a] = x
    end
  end
end

cnt = Hash.new(0)
H.times do |x|
  W.times do |y|
    c = grid[x][y]
    cnt[c] += 1
  end
end

puts cnt.size
cnt.keys.sort.each do |c|
  puts "#{c} #{cnt[c]}"
end
