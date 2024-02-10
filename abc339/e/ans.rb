def geti
  gets.chomp.to_i
end

def getis
  gets.chomp.split(" ").map(&:to_i)
end
N, D = getis
A = getis


T = (5 * 10**5) * 2 + 1

class SegTree
  def initialize(n, init: 0)
    size = 1
    while n > size
      size *= 2
    end
    @n = size
    @init = init
    @arr = Array.new(@n * 2) { init }
  end

  def update(i, v)
    i += @n-1
    @arr[i] = v
    while i > 0
      i = (i - 1) / 2
      @arr[i] = v if @arr[i] < v
    end
  end

  # [a, b)
  def query(a, b)
    query_sub(a, b, 0, 0, @n)
  end

  def query_sub(a, b, k, l, r)
    if r <= a || b <= l
      return @init
    elsif (a <= l && r <= b)
      return @arr[k]
    else
      m = (l + r)/2
      vl = query_sub(a, b, k*2+1, l, m)
      vr = query_sub(a, b, k*2+2, m, r)
      # if vl.nil? || vr.nil?
      #   p [a, b, k, l, r]
      #   require "pry"
      #   binding.pry
      # end
      [vl, vr].max
    end
  end
end


# 最後が i のときの長さ
s = SegTree.new(T)
A.each do |a|
  l = a - D
  l = 0 if l < 0
  r = a + D + 1
  r = T if r > T
  m = s.query(l, r)

  # puts "the max of [#{l}, #{r}) is #{m}"
  # puts "update #{a} with #{m + 1}"

  s.update(a, m + 1)
end

puts s.query(0, T)
