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

def cross_range(arr, l, r, m)
  ls_max = nil
  l_max = m
  sum = 0
  m.downto(l) do |i|
    sum += arr[i]
    if ls_max.nil? || sum > ls_max
      ls_max = sum
      l_max = i
    end
  end

  rs_max = nil
  r_max = m + 1
  sum = 0
  (m+1).upto(r) do |i|
    sum += arr[i]
    if rs_max.nil? || sum > rs_max
      rs_max = sum
      r_max = i
    end
  end

  [l_max, r_max, ls_max + rs_max]
end

def max_range(arr, l = 0, r = arr.size - 1)
  return [l, r, arr[l]] if l == r
  mid = (l+r)/2
  l_l, l_r, l_sum = max_range(arr, l, mid)
  r_l, r_r, r_sum = max_range(arr, mid + 1, r)
  c_l, c_r, c_sum = cross_range(arr, l, r, mid)

  ma = [l_sum, r_sum, c_sum].max
  case ma
  when l_sum then [l_l, l_r, l_sum]
  when r_sum then [r_l, r_r, r_sum]
  when c_sum then [c_l, c_r, c_sum]
  end
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

  # require 'pry'; binding.pry
  # loop
  arr += arr
  left = 0
  max = nil
  while left < lsize
    right = left + k - 1
    _ , _, s = max_range(arr, left, right)
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
