N = 100
ans = 50

puts N

M = gets.chomp.to_i

dh = Hash.new {|h,k| h[k] = []}
M.times do |idx|
  arr = gets.chomp.split(" ").map(&:to_i)
  arr[1..].each do |drink|
    dh[drink] << idx
  end
end

broken = dh[ans].to_h { |e| [e, true] }
s = M.times.map do |h|
  if broken[h]
    '1'
  else
    '0'
  end
end.join("")

puts s
