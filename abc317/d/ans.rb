N = gets.to_i
tz = 0
total_z = 0
div = []
N.times.map do
  x, y, z = gets.chomp.split(' ').map(&:to_i)
  total_z += z
  if x > y
    tz += z
  else
    th = (x+y)/2 + 1
    diff = th - x
    div << { required: diff, delta: z }
  end
end

def bin_serach(r, l)
  loop do
    i = (r + l) / 2
    if i == l || i == r
      break
    end
    f = yield i
    if f
      l = i
    else
      r = i
    end
  end

  l
end

threshold = total_z / 2 + 1

if tz >= threshold
  puts 0
else
  dp = {}
  dp[0] = 0
  div.each do |h|
    a = dp.entries
    a.each do |k, v|
      nk=k+h[:delta]
      nv = v + h[:required]
      if dp[nk].nil? || dp[nk] > nv
        dp[nk] = nv
      end
    end
  end
  min = nil
  dp.each do|k, v|
    next if k < threshold - tz

    min = v if min == nil || min > v
  end

  puts min
end
