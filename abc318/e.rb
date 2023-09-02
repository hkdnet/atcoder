N = gets.to_i
as = gets.chomp.split(" ").map(&:to_i)
cnt = Hash.new {|h,k|h[k] = []}
N.times do |i|
  cnt[as[i]] << i
end
ans = 0
cnt.each do |_, arr|
  ds = arr.each_cons(2).map do |i, k|
    k - i -1
  end
  ans += ds.sum * ds.size
end

puts ans
