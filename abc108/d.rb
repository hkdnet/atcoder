L = gets.to_i

N = [(L-1).to_s(2).size + 1, 20].min

edges = []

1.upto(N-1) do |i|
  edges << { s: i, g: i+1, l: 0 }
end

N.downto(2) do |g|
  len = 1 << (N-g)
  if (len * 2 - 1) <= L
    edges << { s: g-1, g: g, l: len }
  end
end

# 最後の辺
if edges[-1][:s] != 1
  len = 1 <<(N-2)
  diff = L - 1 - len
  if diff == 0
    edges << { s: 1, g: N, l: len }
  else
    cnt = 1
    while diff > 1
      diff /= 2
      cnt += 1
    end
    edges << { s: 1, g: N - cnt, l: len }
  end
else
  max = 1 << (20-1)
  diff = L - max
  if diff == 0
    edges << { s: 1, g: N, l: max }
  elsif diff > 0
    g = N - diff.to_s(2).size
    edges << { s: 1, g: g, l: max }
  end
end

puts "#{N} #{edges.size}"
edges.each do |edge|
  puts "#{edge[:s]} #{edge[:g]} #{edge[:l]}"
end
