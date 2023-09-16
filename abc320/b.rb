S = gets.chomp

def parindorome?(s, l, r)
  while l <= r
    if s[l] != s[r]
      return false
    end
    l += 1
    r -= 1
  end

  true
end

h = {}
chars = S.split("")
chars.each.with_index do |c, i|
  h[c] ||= []
  h[c] << i
end

max = 1
chars.size.times do |i|
  c = chars[i]
  arr = h[c]
  ii = arr.find_index(i)
  ji = arr.size - 1
  while ii < ji
    j = arr[ji]
    if parindorome?(S, i, j)
      tmp = j - i + 1
      max = tmp if max < tmp
    end
    ji -= 1
  end
end

puts max
