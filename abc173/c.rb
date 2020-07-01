h, w, k = gets.chomp.split(' ').map(&:to_i)
hw = h.times.map do
  gets.chomp.split('')
end

def dfs(hw, h, i, k, used)
  return {} if k < 0
  return {} if i >= h + hw.first.size

  ans = dfs(hw, h, i + 1, k, used)

  u = used + [i]
  idx = i >= h ? i - h : i
  diff = if i >= h
    # w
    d = 0
    h.times do |hi|
      next if used.include?(hi)
      d += 1 if hw[hi][idx] == '#'
    end
    d
  else
    # h
    hw[idx].count { |e| e == '#' }
  end

  nk = k - diff

  if nk == 0
    key = u.join('$')
    ans[u.join('$')] = 1
  end
  ans.merge!(dfs(hw, h, i + 1, nk, u))

  ans
end

cnt = hw.sum { |arr| arr.count { |e| e == '#' } }

h = dfs(hw, h, 0, cnt - k, [])
# p h
ans = h.size
if cnt == k
  ans += 1
end
puts ans
