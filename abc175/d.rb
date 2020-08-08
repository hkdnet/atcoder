def g
  gets.chomp.split(' ').map(&:to_i)
end

def build_loops(n, ps)
  cnt = 0
  ret = []
  while cnt != n
    arr = []
    idx = 0
    idx += 1 while ps[idx] == -1

    while idx < n && ps[idx] != -1
      nex = ps[idx] - 1
      arr << nex
      ps[idx] = -1
      idx = nex
      cnt += 1
    end

    ret << arr
  end

  ret
end

def max_range(arr, l = 0, r = arr.size - 1)
  return 0 if r < l

  max = tmp = arr[l]
  (l+1).upto(r) do |idx|
    e = arr[idx]
    tmp = [e, tmp + e].max
    max = [max, tmp].max
  end

  max
end

def best_point(k, l, cs)
  sum = 0
  lsize = l.size
  arr = l.map { |e| n = cs[e]; sum += n; n }
  point = 0
  if sum > 0
    point += sum * (k / lsize)
    k = k % lsize
  else
    if k >= lsize
      k = lsize
    end
  end

  # loop
  arr += arr
  left = 0
  max = nil
  while left < lsize
    right = left + k - 1
    s = max_range(arr, left, right)
    if max.nil? || s > max
      max = s
    end
    left += 1
  end

  point + max
end

# puts best_point(3, [1, 0], [10, -7])
# puts best_point(58, [8, 9, 4, 7, 1, 0], %w[695279662 988782657 -119067776 382975538 -151885171 -177220596 -169777795 37619092 389386780 980092719].map(&:to_i))
# exit 0

n, k = g
ps = g
cs = g

max = nil
loops = build_loops(n, ps)
loops.each do |loo|
  pt = best_point(k, loo, cs)
  if max.nil? || pt > max
    max = pt
  end
end

puts max
