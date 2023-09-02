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
  ms = [ds.size]
  d = ds.size - 2
  i = 0
  while d > 0
    ms << ms.last + d
    d -= 2
  end
  if ds.size % 2 == 0
    ms += ms.reverse
  else
    ms += ms[0..-2].reverse
  end

  ds.each_with_index do |d, i|
    if i >= ms.size
      i = ms.size - 1 - (i - ms.size - 1)
    end
    ans += d * ms[i]
  end
end

puts ans
