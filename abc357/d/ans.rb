def getl = gets.chomp
def geti = getl.to_i
def getis = getl.split(" ").map(&:to_i)
def getis1 = getl.split(" ").map { |e| e.to_i - 1 }

MOD = 998244353
N = geti
r = 10 ** N.to_s.size

def modinv(a)
  b = MOD
  u = 1
  v = 0
  while b != 0
    t = a / b
    a -= t * b
    a, b = b, a
    u -= t*v
    u, v = v, u
  end
  u %= MOD
  if u < 0
    u += MOD
  end
  u
end

class PowCalc
  def initialize(base)
    @base = base
    @memo = {}
  end

  def pow(b)
    @memo[b] ||= calc_pow(b)
  end

  private

  def calc_pow(b)
    return 1 if b == 0
    return @base if b == 1
    return @base ** 2 if b == 2

    b1 = b/2
    p1 = calc_pow(b1)
    b2 = b - b1
    p2 = calc_pow(b2)
    p1 * p2 % MOD
  end
end

pc = PowCalc.new(r)


ans = r.pow(N, MOD)
ans *= N
ans %= MOD
ans *= modinv(r-1)

puts ans
