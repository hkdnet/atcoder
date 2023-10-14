MAX = 10_000_000_000_000
h = {}
1.upto(MAX) do |n|
  a = n ** 2
  break if a >= MAX
  h[a] = true
end

p h.size
