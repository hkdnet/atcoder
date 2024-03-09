def geti
  gets.chomp.to_i
end

def getis
  gets.chomp.split(" ").map(&:to_i)
end

T = gets.chomp
N = geti

arr = N.times.map do
  _, *ss = gets.chomp.split(" ")
  ss
end

dp = Hash.new(10000000000)
dp[0] = 0
arr.each do |ss|
  tmp = Hash.new(10000000000)
  dp.each do |k, v|
    nv = v + 1

    tmp[k] = v if v < tmp[k] # do nothing

    # pick 1 if possible
    ss.each do |s|
      su = T[k, s.size]
      # p [s, su]
      if su == s
        nx = k + s.size
        tmp[nx] = nv if nv < tmp[nx]
      end
    end
  end
  # p tmp
  dp = tmp
end

ans = dp[T.size]
if ans == 10000000000
  puts "-1"
else
  puts ans
end
