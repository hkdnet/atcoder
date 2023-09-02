N = gets.to_i

cnt = Hash.new{|h, k| h[k]=0}
N.times.to_a.permutation(2).each do |a, b|
  i = a + 0.5
  while i < b
    cnt[i] += 1
    i += 1
  end
end
p cnt
