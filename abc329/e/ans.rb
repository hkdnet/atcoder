def geti
  gets.chomp.to_i
end

def getis
  gets.chomp.split(" ").map(&:to_i)
end

N, M = getis
S = gets.chomp
T = gets.chomp
CS = T.chars
idx = S.index(T)
if idx
  # <-
  l = idx
  CS.size.times do |d|
    nl = l - d
    if
  end
else
  puts "No"
end


q = []
f = false
q << [0, true]
while !q.empty?
  cur, use_last = q.pop
  puts "Now at #{cur}"
  if cur == S.size && use_last
    f = true
    break
  end
  next if cur > S.size
  candidates = {}

  T.size.times do |l|
    break if cur - l < 0
    break if l >= 1 && !use_last

    d = 0
    while l + d < T.size
      break if T[l+d] != S[cur+d]
      d += 1
    end
    d -= 1
    if d < 0
      next
    end
    puts "total #{d} made"
    0.upto(d) do |dd|
      candidates[l + dd] = (l + dd == T.size - 1)
    end
  end


  puts "kouho"
  p candidates
  candidates.each do |k, v|
    next if k + 1 <= cur
    puts "#{cur} -> #{k + 1}, use_last #{v}"
    q << [k + 1, v]
  end
end


puts(f ? "Yes" : "No")
