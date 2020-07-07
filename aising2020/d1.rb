n = gets.chomp.to_i
x = gets.chomp
xi = x.to_i(2)

h = {}
t = 2 ** (n-1)
n.times do |i|
  if x[i] == '0'
    h[i] = xi + t
  else
    h[i] = xi - t
  end

  t /= 2
end

def exec(n)
  c = n.to_s(2).count('1')
  n % c
end
arr = Array.new(n, 0)

while !h.empty?
  h.each do |(k, v)|
    h[k] = exec(v)
    arr[k] += 1
    if h[k] == 0
      h.delete(k)
    end
  end
end

arr.each do |a|
  puts a
end


