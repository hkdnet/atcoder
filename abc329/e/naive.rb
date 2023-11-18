def geti
  gets.chomp.to_i
end

def getis
  gets.chomp.split(" ").map(&:to_i)
end

N, M = getis
S = gets.chomp
T = gets.chomp.chars

start = '#' * N
used = Set.new
q = [start]
used << start
f = false
while !q.empty? do
  cur = q.pop
  if cur == S
    f = true
    break
  end
  l = 0
  while l + T.size <= S.size
    a = cur.split("")
    T.each.with_index do |c, d|
      a[l + d] = c
    end

    s = a.join("")
    if !used.include?(s)
      q << s
      used << s
    end

    l += 1
  end
end

if f
  puts "Yes"
else
  puts "No"
end
