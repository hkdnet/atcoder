# @return [Integer] an integer of [l, r)
def randi(l, r)
  rand(r-l) + l
end

# arr.shuffle
# arr.sample
# arr.sample(n)
N = 200_000
M = 200_000
puts [N, M].join(" ")
A = N.times.map { randi(1, 200_000) }
puts A.join(" ")

edges = Hash.new {|h, k| h[k] = {} }
cnt = 0
1.upto(N) do |a|
  rand()
  break if cnt == M
end

if cnt != M
  raise "nya-"
end

edges.each do |a, v|
  v.each do |b, _|
    puts [a, b].join(" ")
  end
end
