def geti
  gets.chomp.to_i
end

def getis
  gets.chomp.split(" ").map(&:to_i)
end

N = geti
v = []
cs = N.times.map do |i|
  a, b = getis.sort
  v[a] = [i, true]
  v[b] = [i, false]
end

def solve(v)
  q = []
  1.upto(N*2) do |i|
    num, f = v[i]
    if f
      q << num
    else
      got = q.pop
      if num != got
        return true
      end
    end
  end

  false
end

if solve(v)
  puts "Yes"
else
  puts "No"
end
