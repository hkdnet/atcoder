def geti
  gets.chomp.to_i
end

def getis
  gets.chomp.split(" ").map(&:to_i)
end

N, Q = getis
arr = Array.new(N) { nil }

class UnionFind
  def initialize(n)
    @arr = n.times.map {|e| e }
    @delta = Array.new(n) { 0 }
  end

  def unite(a, b, d)
    ra, ra_a_d = root(a)
    rb, rb_b_d = root(b)
    if ra == rb
      if ra_a_d - rb_b_d == d
        @arr[b] = ra
        @delta[b] =
      else
        return false
      end
    else

    end
    rad = a + ad
    rbd = b + bd

  end

  def root(n, d = 0)
    if @arr[n] == n
      n, d
    else
      r, dd = root(@arr[n], d + @delta[n])
      @delta[r] = dd
      r, dd
    end
  end
end

ans = []
Q.times do |q|
  q = q + 1
  a, b, d = getis
  a = a - 1
  b = b - 1

  if true
    ans << q
  end
end

puts ans.join(" ")
