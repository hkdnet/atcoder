# @return [Integer] an integer of [l, r)
def randi(l, r)
  rand(r-l) + l
end

# arr.shuffle
# arr.sample
# arr.sample(n)

H = 200
W = 200
start = [0, 0]
goal = [H-1, W-1]
puts "#{H} #{W}"
H.times do |h|
  s = ""
  W.times do |w|
    if [h, w] == start
      s << "S"
    elsif [h, w] == goal
      s << "T"
    else
      s << '..........#'.chars.sample
    end
  end
  puts s
end
N = 300
puts "1 1 #{randi(1, 10)}"
(N-1).times do
  h = randi(1, H)
  w = randi(1, W)
  puts "#{h} #{w} #{randi(1, 100)}"
end
