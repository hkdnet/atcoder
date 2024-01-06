# @return [Integer] an integer of [l, r)
def randi(l, r)
  rand(r-l) + l
end
N = 10**5
M = 10**5
A = N.times.map { randi(1, 10**9) }
B = M.times.map { randi(1, 10**9) }
h = Hash.new {|h,k|h[k] = {}}
N.times do
  a = randi(1, 10**5)
  b = randi(1, 10**5)
  next if a == b
  h[a][b] = true
  h[b][a] = true
end
L = h.values.map(&:size).sum / 2
puts "#{N} #{M} #{L}"
puts "#{A.join(" ")}"
puts "#{B.join(" ")}"
h.each do |c, v|
  v.each do |d, _|
    if c < d
      puts "#{c} #{d}"
    end
  end
end

# arr.shuffle
# arr.sample
# arr.sample(n)
